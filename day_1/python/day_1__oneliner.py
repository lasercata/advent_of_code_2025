#!/usr/bin/env python3
# -*- coding: utf-8 -*-

n = 50; print(len([1 for k in open('../data/data_1.txt').read().replace('L', '-').replace('R', '').strip().split('\n') if (n := ((n + int(k)) % 100)) == 0]))
