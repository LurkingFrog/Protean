table! {
    invoices (guid) {
        guid -> Varchar,
        organization -> Varchar,
        status -> Varchar,
        balance -> Float8,
        credits -> Float8,
        debits -> Float8,
        date_created -> Timestamp,
        due_on -> Nullable<Timestamp>,
        terms -> Varchar,
        last_modified -> Timestamp,
    }
}

table! {
    organizations (guid) {
        guid -> Varchar,
        pretty_id -> Varchar,
        name -> Varchar,
        department -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        address1 -> Varchar,
        address2 -> Nullable<Varchar>,
        city -> Varchar,
        state -> Varchar,
        zip -> Varchar,
        parent -> Nullable<Varchar>,
        members -> Array<Text>,
        children -> Array<Text>,
        payment_terms -> Varchar,
    }
}

table! {
    patients (guid) {
        guid -> Varchar,
        species -> Varchar,
    }
}

table! {
    paymentappliedto (guid) {
        guid -> Varchar,
        payment -> Nullable<Varchar>,
        amount -> Nullable<Numeric>,
        submission -> Nullable<Varchar>,
        invoice -> Nullable<Varchar>,
    }
}

table! {
    payments (guid) {
        guid -> Varchar,
        payer -> Varchar,
        payee -> Varchar,
        method -> Varchar,
        amount -> Float8,
        comment -> Nullable<Text>,
        date_received -> Timestamp,
    }
}

table! {
    submissionlineitem (guid) {
        guid -> Varchar,
        submission -> Nullable<Varchar>,
        name -> Varchar,
        quantity -> Float8,
        quantity_unit -> Nullable<Varchar>,
        price_per_unit -> Nullable<Numeric>,
    }
}

table! {
    submissions (guid) {
        guid -> Varchar,
        accession_number -> Varchar,
        submitting_org -> Varchar,
        service_line_items -> Nullable<Varchar>,
        species -> Varchar,
        pet_name -> Nullable<Varchar>,
        total -> Nullable<Numeric>,
        received -> Timestamp,
        finalized -> Nullable<Timestamp>,
        paid_on -> Nullable<Timestamp>,
    }
}

table! {
    tests (guid) {
        guid -> Varchar,
        value -> Varchar,
    }
}

table! {
    users (guid) {
        guid -> Varchar,
        salutation -> Nullable<Varchar>,
        first -> Varchar,
        middle -> Nullable<Varchar>,
        last -> Varchar,
        credentials -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
    }
}

joinable!(invoices -> organizations (organization));
joinable!(paymentappliedto -> invoices (invoice));
joinable!(paymentappliedto -> payments (payment));
joinable!(paymentappliedto -> submissions (submission));
joinable!(submissionlineitem -> submissions (submission));

allow_tables_to_appear_in_same_query!(
    invoices,
    organizations,
    patients,
    paymentappliedto,
    payments,
    submissionlineitem,
    submissions,
    tests,
    users,
);
