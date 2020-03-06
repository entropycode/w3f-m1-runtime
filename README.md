# Feedback Module

The feedback module facilitate users to create polls with certain options and expiration times.

Please see MANUAL for build instructions and additional information.

#### Poll

Each poll is created with the following logic: 

- Each poll has a `poll_id` as their unique identifier which is incremented per poll created
- All options of a poll is hashed and stored
- Expiration time is calculated by adding the input arg `open_for`, in milliseconds, to the time from the runtime timestamp pallet

#### Respond

An account can respond to the poll if:

- it has not responded to the same poll (checked in `Entries`)
- the poll has not expired

The response is added to `Responses` by incrementing the tally of the option by 1

#### Seal

After a poll has expired, anyone can call seal which:

- iterates through the responses tally of the polls 
- set the most popular option to be the result of the poll



Copyright © 2019-2020 Entropy Labs