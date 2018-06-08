# -*- coding: utf-8 -*-
from locators import *
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import NoSuchElementException
import time



driver = webdriver.Chrome()

driver.get(testdev)
driver.implicitly_wait(5)
assert u"Storiqa" in driver.title

def click(adr):
    driver.find_element_by_xpath(adr).click()
    

click(user)