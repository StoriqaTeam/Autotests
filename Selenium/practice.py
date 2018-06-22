# -*- coding: utf-8 -*-
from locators import *
import registration as r
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
import time

passed_test_count = 0
failed_test_count = 0

# Tests with parametrs
test_registration = r.Registration('tester', 'testoviy', regmail, 'qwe123QWE')
test_authorization = r.Authorization('tester@storiqa.com', 'qwe123QWE')
test_user_profile_update = r.User()

test_suite = [test_authorization.start(), test_user_profile_update.profile()]

if __name__ == "__main__":
    #test_registration.start()
    if all in test_suite is True:
        passed_test_count += 1
    else:
        failed_test_count += 1
    print ('Test finished with %s PASSED and %s FAILED' % (passed_test_count, failed_test_count))
    input('END')