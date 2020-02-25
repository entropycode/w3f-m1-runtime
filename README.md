---
title: "Feedback Pallet"
---

The feedback pallet facilitate users to create polls with certain options and expiration times, this is an example pallet used to demonstrate this [CLI tool](). This documents the logic behind the feedback pallet, as it's own crate, that can be used along with other Pallets in substrate.

The code for this pallet can be found [in this repository]().


#### Poll

Each poll is created with the following logic: 

- Each poll has a `poll_id` as their unique identifier which is incremented per poll created
- All options of a poll is hashed and stored
- Expiration time is calculated by adding the input arg `open_for`, in milliseconds, to the time from the runtime timestamp pallet

#### Respond

An account can respond to poll and the pallet makes sure that:

- An account have not responded to the same poll, checked in `Entries`
- The poll is still active and has not expired

The response is added to `Responses` by incrementing the tally of the option by 1

#### Seal

After a poll has expired, anyone can call seal which:

- iterates through the responses tally of the polls 
- set the most popular option to be the result of the poll




