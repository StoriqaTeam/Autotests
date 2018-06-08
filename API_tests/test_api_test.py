#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import requests
import json
import os
import graph_queries as q
from datetime import datetime




if os.getenv('GRAPHQL_URL'):
    url = os.environ['GRAPHQL_URL']
else: url = 'https://nightly.stq.cloud/graphql'


lst = q.queries.items()
print (type(lst))
