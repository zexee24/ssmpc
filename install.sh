#!/bin/bash

trunk build --release
sudo rm -rf /srv/http/*
sudo cp dist/* /srv/http
sudo systemctl start httpd.service
