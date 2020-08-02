-- Your SQL goes here

CREATE TABLE Tests (
  guid VARCHAR PRIMARY KEY,
  value VARCHAR NOT NULL
);

CREATE TABLE Users (
  guid VARCHAR PRIMARY KEY,
  salutation VARCHAR,
  first VARCHAR NOT NULL,
  middle VARCHAR,
  last VARCHAR NOT NULL,
  credentials VARCHAR,
  title VARCHAR,
  email VARCHAR,
  phone VARCHAR
);


CREATE TABLE Patients (
  guid VARCHAR PRIMARY KEY,
  species VARCHAR NOT NULL
);


CREATE TABLE Organizations (
  guid VARCHAR PRIMARY KEY,
  pretty_id VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  department VARCHAR,
  email VARCHAR,
  phone VARCHAR,
  address1 VARCHAR NOT NULL,
  address2 VARCHAR,
  city VARCHAR NOT NULL,
  state VARCHAR NOT NULL,
  zip VARCHAR NOT NULL,
  parent VARCHAR REFERENCES Organizations(guid),
  --  TODO: Change this to M-M w/users
  members TEXT[] NOT NULL,
  --  TODO: Change this to M-M w/self
  children TEXT[] NOT NULL,
  payment_terms VARCHAR NOT NULL
);

CREATE TABLE Submissions (
  guid VARCHAR PRIMARY KEY,
  accession_number VARCHAR NOT NULL,
  submitting_org VARCHAR NOT NULL,
  service_line_items VARCHAR,
  species VARCHAR NOT NULL,
  pet_name VARCHAR,
  total NUMERIC(12, 2),
  received timestamp NOT NULL,
  finalized timestamp,
  paid_on timestamp
);

CREATE TABLE SubmissionLineItem (
  guid VARCHAR PRIMARY KEY,
  submission VARCHAR REFERENCES Submissions,
  name VARCHAR NOT NULL,
  quantity FLOAT NOT NULL,
  quantity_unit VARCHAR,
  price_per_unit NUMERIC(12, 2)
);


CREATE TABLE Payments (
  guid VARCHAR PRIMARY KEY,
  payer VARCHAR REFERENCES Organizations(guid) NOT NULL,
  payee VARCHAR REFERENCES Organizations(guid) NOT NULL,
  method VARCHAR NOT NULL,
  amount FLOAT NOT NULL,
  comment TEXT,
  date_received timestamp NOT NULL
);


CREATE TABLE Invoices (
  guid VARCHAR PRIMARY KEY,
  organization VARCHAR REFERENCES Organizations(guid) NOT NULL,
  status VARCHAR NOT NULL,
  balance FLOAT NOT NULL,
  credits FLOAT NOT NULL,
  debits FLOAT NOT NULL,
  date_created timestamp NOT NULL,
  due_on timestamp,
  terms VARCHAR NOT NULL,
  last_modified timestamp NOT NULL
);

CREATE TABLE PaymentAppliedTo (
  guid VARCHAR PRIMARY KEY,
  payment VARCHAR REFERENCES Payments(guid),
  amount NUMERIC(12, 2),
  submission VARCHAR REFERENCES Submissions(guid),
  invoice VARCHAR REFERENCES Invoices(guid)
);

ALTER TABLE PaymentAppliedTo
ADD CONSTRAINT chk_only_one_is_not_null CHECK (num_nonnulls(submission, invoice) = 1);
-- TODO: Add Constraint that Amount cannot be more than the payment


