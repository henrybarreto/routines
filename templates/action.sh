#!/bin/sh
# `action.sh` is a templated file intend to be the draft for a specific action.
# A action should be created by the end user.

# Called before up and down's execution.
# Before can be used to store data for the `down` function, as a example.
# It returns 0 if it could execute the action and 1 for any error.
before() { return 0; }

# Called to do a event action.
# It returns 0 if it could execute the action and 1 for any error.
# It can receive any values to be executed.
up() { return 0; }

# Called to undo a event action.
# It returns 0 if it could execute the action and 1 for any error.
# It can receive any values to be executed.
down() { return 0; }

# Called after up and down's execution.
after() { return 0; }