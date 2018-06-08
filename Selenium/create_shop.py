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


class Create_store(unittest.TestCase):
	
	print 'Create store test'
	#driver = webdriver.PhantomJS()
	driver = webdriver.Firefox()
	
	def test_create_store(self):
		driver = self.driver
		driver.get("https://sb1.storiqa.com/start")
		driver.implicitly_wait(5)
		self.assertIn(u"Storiqa", driver.title)
		elem = driver.find_element(By.LINK_TEXT, login).click()
		elem = driver.find_element(By.XPATH, email)
		elem.click()
		elem.send_keys('Tester1@mail.ru')
		elem = driver.find_element_by_xpath (pwd)
		elem.click()
		elem.send_keys('qwe123QWE')
		elem = driver.find_element_by_xpath(enter).click()
		assert(driver.find_element_by_xpath("//div [@class='user-menu']"))
		driver.find_element_by_link_text(create_store).click()
		elem = WebDriverWait(driver, 3).until(EC.element_to_be_clickable((By.XPATH, base_tariff)))
		elem.click()
		driver.find_element(By.XPATH, dalee).click()
		elem =  driver.find_element(By.XPATH, name)
		elem.click()
		elem.send_keys(u'MagazinМагазин')
		time.sleep(1)
		elem =  driver.find_element(By.XPATH, discription)
		elem.click()
		elem.send_keys(u'MagazinМагазин')
		time.sleep(1)
		elem = driver.find_element(By.XPATH, location)
		elem.click()
		elem.send_keys(u'Москва', Keys.ENTER)
		elem = driver.find_element(By.XPATH, phone)
		elem.click()
		elem.send_keys(u'89035775634')
		driver.find_element(By.XPATH, submit).click()

if __name__ == "__main__":
	unittest.main()

