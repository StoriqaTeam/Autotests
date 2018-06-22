# -*- coding: utf-8 -*-
import pytest
from locators import *
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import NoSuchElementException
import time

print 'Registration test'
# driver = webdriver.PhantomJS()
# driver = webdriver.Firefox()
# driver = webdriver.Chrome()

def test_example(selenium):
    selenium.get(testdev)
    return selenium


