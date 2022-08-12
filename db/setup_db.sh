#!/usr/bin/env bash

cd $(dirname "$0")
sqlite3 ../sampler.db < db.sql
