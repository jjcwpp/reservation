# Core Reservation Service

- Feature Name: core-reservation-service
- Start Date: 2022-11-14 17:28:13

## Summary

A core reservation service that solves the problem of reserving a resource for  a period of time. We leverage postgres EXCLUDE constraints to ensure that only one reservation can be made at a given time.

## Motivation

We need a common solution for various reservation requirements: 1) calendar booking; 2) hotel/room booking; 3) meeting room booking; 4) parking lot booking; 5) etc. Repeatedly building features for these requirements is a waste of time and resources. We should have a common solution that can be used by all teams.

## Guide-level explanation

### architecture

### interface

We would use gPRC as a service interface. Below is the proto definition:

```proto
enum ReservationStatus {
    UNKNOWN =0;
    PENDING =1;
    CONFIRMED =2;
    BLOCKED =3;
}

enum ReservationUpdateType {
    UNKNOWN =0;
    CREATE =1;
    UPDATE =2;
    DELETE =3;
}

message Reservation {
    string id=1;
    string user_id=2;
    ReservationStatus status=3;

    string resource_id=4;
    google.protobuf.Iimestamp start =5;
    google.protobuf.Iimestamp end =6;

    string note=7;
}

message ReserveRequest{
    Reservation reservation=1;
}

message ReserveResponse {
    Reservation reservation=1;
}

message UpdateRequest {
    string note =1;
}

message UpdateResponse {
    Reservation reservation=1;
}

message ConfirmRequest {
    string id =1;
}

message ConfirmResponse {
    Reservation reservation=1;
}

message CancelRequest {
    string id =1;
}

message CancelResponse {
    Reservation reservation=1;
}

message GetRequest {
    string id =1;
}

message GetResponse {
    Reservation reservation=1;
}

message QueryRequest {
    string resource_id =1;
    string user_id =2;

    ReservationStatus status=3;
    google.protobuf.Timestamp start=4;
    google.protobuf.Timestamp end=5;
}

message ListenRequest {}

message ListenResponse {
    int8 op=1;
    Reservation reservation=2;
}

service ReservationService {
    rpc reserve(ReserveRequest) returns (ReserveResponse);
    rpc confirm(ConfirmRequest) returns (ConfirmResponse);
    rpc update(UpdateRequest) returns (UpdateResponse);
    rpc cancel(CancelRequest) returns (CancelResponse);
    rpc get(GetRequest) returns (GetResponse);
    rpc query(QueryRequest) returns (stream Reservation);
    rpc listen(ListenRequest) returns (stream ListenResponse);
}
```

### database schema

We use postgres as the database. Below is the schema:

```sql
CREATE SCHEMA rsvp;
CREATE TYPE rsvp.reservation_status AS ENUM('unknown', 'pending', 'confirmed','blocked');
CREATE TYPE rsvp.reservation_update_type AS ENUM('unknown', 'created', 'updated','delete');

CREATE TABLE IF NOT EXISTS rsvp.reservations (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    user_id varchar(64) NOT NULL,
    status rsvp.reservation_status NOT NULL DEFAULT 'pending',
    resource_id VARCHAR(64) NOT NULL,
    timespan TSTZRANGE NOT NULL,
    note text,

    CONSTRAINT reservations_pkey PRIMARY KEY (id),
    CONSTRAINT reservations_conflict EXCLUDE USING gist(resource_id WITH =, timespan WITH &&),
);

CREATE INDEX reservations_resource_id_idx ON rsvp.reservations(resource_id);
CREATE INDEX reservations_user_id_idx ON rsvp.reservations(user_id);

CREATE OR REPLACE FUNCTION rsvp.query(uid text, rid text, during TSTZRANGE)
RETURNS TABLE rsvp.reservations AS $$ $$ LANGUAGE plpgsql;

CREATE TABLE rsvp.reservation_changes (
    id SERIAL NOT NULL,
    reservation_id uuid NOT NULL,
    op rsvp.reservation_update_type NOT NULL,
);

CREATE OR REPLACE FUNCTION rsvp.reservation_trigger() RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO rsvp.reservation_changes(resource_id, op) VALUES (NEW.id, 'CREATE');
    ELSIF TG_OP = 'UPDATE' THEN
        IF OLD.status <> NEW.status THEN
            INSERT INTO rsvp.reservation_changes(resource_id, op) VALUES (NEW.id, 'UPDATE');
        END IF;
    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO rsvp.reservation_changes(resource_id, op) VALUES (NEW.id, 'DELETE');
    END IF;
    NOTIFY reservation_update;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER reservation_trigger
    AfTER INSERT OR UPDATE OR DELETE ON rsvp.reservations
    FOR EACH ROW EXECUTE PROCEDURE rsvp.reservations_trigger();

```
