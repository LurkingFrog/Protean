#! /usr/bin/env zsh


export CLICOLOR_FORCE=1
export RUST_BACKTRACE=1;
export RUST_LOG=debug,test=debug;
# export RUSTFLAGS="-Z macro-backtrace -Z debug-macros"


# WORKDIR=~/Foundry/Protean/protean_examples/diesel_updates
WORKDIR=~/Foundry/Protean/

# An easier to read variable for calling docker compose. We'll set this programmatically
COMPOSE_FILE=$WORKDIR/docker/fhl.docker-compose.yml
COMPOSE=

#RipGrep is used to parse files - easier to extract data from the compose files
RG=

# Standard PSQL calls to the postgres database. Set when we start Postgres
PSQL=
PG_USER=
PG_PASS=


# If Diesel is missing on ubuntu:
#> sudo apt install libpq-dev
#> sudo ln -s /usr/lib/x86_64-linux-gnu/libzmq.so.5 /usr/lib/x86_64-linux-gnu/libpq.so
#> cargo install diesel_cli --no-default-features --features postgres --verbose

SEP="\n\n<---------------------->\n\n"

# Ensure we have a value of docker compose
function get_docker_compose {
  echo "\n\tFinding Docker Compose"
  COMPOSE=($(command -v docker-compose) -f $COMPOSE_FILE)
  if [ $? -ne 0 ]; then
    echo "\tDocker-compose not installed: Attempting to install it now";
    sudo curl \
      -L "https://github.com/docker/compose/releases/download/1.26.2/docker-compose-$(uname -s)-$(uname -m)" \
      -o /usr/local/bin/docker-compose
    sudo chmod 755 /usr/local/bin/docker-compose


    COMPOSE=($(command -v docker-compose) -f $COMPOSE_FILE)
    if [ $? -ne 0 ]; then
      echo "\tFailed to get docker-compose"
       exit $?
    fi
  fi
  echo "\tFinished installing docker-compose."
  echo "\tUsing docker-compose command $COMPOSE"
}

function get_ripgrep {
  echo "${SEP}getting ripgrep"
  RG=$(command -v rg)
  if [ $? -ne 0 ]; then
    echo "\tRipgrep not installed: Attempting to install it now";
    $(sudo apt install ripgrep)

    RG=$(command -v rg)
    if [ $? -ne 0 ]; then
      echo "\tFailed to get ripgrep"
      exit $?
    fi
  fi
  echo "\tFinished installing ripgrep."
  echo "\tUsing ripgrep command $RG"
}


# Simple function to make sure the postgres container is running before anything else
function wait_postgres {
  AWAKE=0
  while [ $AWAKE -lt 60 ]; do
    # Query Postgres

    RESULT=$($PSQL -c 'SELECT 1')
    if [[ $RESULT =~ '1 row' ]]; then
      AWAKE=3000
      echo "\tPostgres is now awake and running"
    else
      echo "\tNot awake yet. Sleeping again.  Time waiting: $AWAKE"
      AWAKE=$(($AWAKE+1))
      sleep 1
    fi
  done
}


function reset_db {
  echo "$SEP"
  # FIXME: If this is changed in the file but the persistent data volume isn't destroyed, this breaks.
  PG_PASS=$(IFS=":" rg "POSTGRES_PASSWORD:\s*(.+)\s*\$" -or '\$1' $COMPOSE_FILE | read RESULT; echo $RESULT)
  PG_PORT=$(IFS=":" rg "\-\s*(\d+)\s*:\s*5432\s*\$" -or '\$1' $COMPOSE_FILE | read RESULT; echo $RESULT)

  # HACK: Very brittle here, as I'm not encoding anything so an @ in the password will break it
  DB_URL=postgres://${PGUSER:-postgres}:${PG_PASS:-password}@localhost:${PG_PORT:-5432}/headless
  echo "\tSetting Postgres connection string to: '$DB_URL'"
  echo "DATABASE_URL=$DB_URL" > $WORKDIR/.env

  echo "\tResetting the DB if it exists"
  diesel database reset --database-url $DB_URL

  echo "\tAnd running the DB setup with migrations"
  diesel setup --database-url $DB_URL \
  && diesel migration run --database-url $DB_URL \
  && diesel migration redo --database-url $DB_URL
  diesel migration list --database-url $DB_URL
}


function rebuild_project {
  echo "${SEP}Building and running the full test\n"
  # cargo run && build_docs
  cargo test --package test_suite test_replicant_full -- --nocapture \
  2> >(sed '/ \(\(backtrace::\)\|\(core::\)\|\(<\?std::\)\|\(rust_begin_unwind\)\)/, +1d') \
  && build_docs
  # cargo run -p diesel_updates \
#
  echo "\n"
}


function pg_init {
  echo "\n\tInitializing postgres container"
  $COMPOSE up -d --remove-orphans postgres

  PSQL=($COMPOSE exec -T postgres psql -U postgres)
  wait_postgres
  reset_db
}

function init {
  echo "${SEP}Running initialization"

  get_docker_compose
  get_ripgrep

  # pg_init

  echo "${SEP}Doing an initial build attempt"
  cargo build

  mkdir -p $WORKDIR/pdfs


}

# function build_book {
#   echo "\nBuilding the book"
#   cd ./the_process_foundry/book
#   mdbook clean \
#   && mdbook build . \
#   && cp -R ./build/* ~/Foundry/lurkingfrog.github.io/the_process_foundry/book
#   cd ../..
# }

function build_docs {
  echo "\nBuilding the documentation"
  cargo doc --no-deps \
  && mkdir -p  ~/Foundry/lurkingfrog.github.io/protean/docs \
  && cp -R target/doc/* ~/Foundry/lurkingfrog.github.io/protean/docs
}

function rebuild_compose {
    echo "${SEP}Rebuilding Docker Compose"
    $COMPOSE down
    $COMPOSE up -d --remove-orphans postgres
    wait_postgres
    reset_db
}

# Remove all the docker containers before exiting
function tearDown {
  echo "All done, tearing down"
  #/usr/bin/docker-compose -f scripts/docker/dev.docker-compose.yml down
}


INIT_DIR=$(pwd)
cd $WORKDIR

# Initialize items like docker compose
init
space=" "
modify="${space}MODIFY${space}"
# And run it the first time before the loop so we don't have to wait for the update
rebuild_project

while true; do
  command -v inotifywait > /dev/null 2>&1 || $(echo -e "InotifyWait not installed" && exit 1)
  EVENT=$(inotifywait -r -e modify \
    $INIT_DIR/watcher.sh \
    $INIT_DIR/Cargo.toml \
    $INIT_DIR/protean \
    $INIT_DIR/tyrell \
    $INIT_DIR/protean_derive \
    $INIT_DIR/protean_examples/diesel_updates \
    $INIT_DIR/test_suite \
  )

  FILE_PATH=${EVENT/${modify}/}
  # echo -e "\nReceived event on file: '${FILE_PATH}'"

  # Root cases
  if [[ $FILE_PATH =~ "watcher.sh" ]]; then
    echo "Matched Watcher.sh. Exiting so we can restart"
    tearDown
    sleep 1
    exit 0


  elif [[ $FILE_PATH =~ ".?/Cargo.toml$" ]]; then
    # Give it time, since VSCode conflicts and wants to do the same
    sleep 5
    rebuild_project

  elif [[ $FILE_PATH =~ "^.?/.+.rs$" ]]; then
    rebuild_project

  elif [[ $FILE_PATH =~ "^.?/.+.sql$" ]]; then
    reset_db

  elif [[ $FILE_PATH =~ "^.?/.+.docker-compose.yml$" ]]; then
    rebuild_compose


  else
    echo -en "No Match on '${FILE_PATH}'': Continuing\n"

  fi
done
