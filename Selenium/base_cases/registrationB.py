# -*- coding: utf-8 -*-
import unittest
from locators import *
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC
from selenium.common.exceptions import NoSuchElementException
import time


class Registration(unittest.TestCase):
    print 'Registration test'
    # driver = webdriver.PhantomJS()
    # driver = webdriver.Firefox()
    driver = webdriver.Chrome()

    # Проверка регистрации
    def test_reg(self):
        driver = self.driver
        # Заходим на сайт и открываем форму регистрации
        driver.get(testdev)
        driver.implicitly_wait(5)
        self.assertIn(u"Storiqa", driver.title)
        driver.find_element_by_xpath(user).click()
        driver.find_element_by_xpath(signup).click()
        time.sleep(1)
        elem = driver.find_element_by_xpath(login)
        elem.click()
        elem.send_keys('Tester')
        elem = driver.find_element_by_xpath(email)
        elem.click()
        elem.send_keys(regmail)
        elem = driver.find_element_by_xpath(pwd)
        elem.click()
        elem.send_keys('qwe123QWE')
        # elem = driver.find_element_by_xpath (pwd_conf)
        # elem.click()
        # elem.send_keys('qwe123QWE+')
        driver.find_element_by_xpath(submitUP).click()
        time.sleep(1)
        alert = driver.switch_to.alert
        alert.accept()
        WebDriverWait(driver, 10).until(EC.presence_of_element_located((By.XPATH, user)))

    ''' def tearDown(self):
		self.driver.close() '''


if __name__ == "__main__":
    unittest.main()