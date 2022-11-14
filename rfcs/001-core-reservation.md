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

```proto

```

### database schema

We use postgres as the database. Below is the schema:

```sql
CREATE SCHEMA rsvp;



CREATE TABLE IF NOT EXISTS rsvp.reservation (
    id uuid NOT NULL DEFAULT uuid_generate_v4(),
    user_id varchar(64) NOT NULL,
    status reservation_status NOT NULL,
    start timestamptz NOT NULL,
    end timestamptz NOT NULL,
    note text
);
```
