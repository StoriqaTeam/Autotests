# -*- coding: utf-8 -*-
from locators import *
import registration as r
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import time


driver = webdriver.Chrome()
driver.get(testdev)
driver.implicitly_wait(5)
assert "Storiqa" in driver.title
print('passed')
r.tap(signup)
input('any key ')
