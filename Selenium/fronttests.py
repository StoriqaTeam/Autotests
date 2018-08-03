#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import os
from locators import *
from selenium.common.exceptions import NoSuchElementException
from selenium import webdriver
from selenium.common.exceptions import ElementNotVisibleException
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as ec
from selenium.webdriver.common.desired_capabilities import DesiredCapabilities

if os.getenv('SELENIUM_URL'):
    url = os.environ['SELENIUM_URL']
else: url = testdev

driver = webdriver.Remote(
    command_executor='http://uxtest.stq.cloud:4444/wd/hub',
    desired_capabilities=DesiredCapabilities.FIREFOX)
#driver = webdriver.Chrome()
driver.maximize_window()
driver.get(url)
driver.implicitly_wait(4)
assert "Storiqa" in driver.title

########################################Interface#########################################
# Exception for failed test
class TestFailException(Exception):
    pass

# click on object
def tap(adr):
    try:
        elem = WebDriverWait(driver, 5).until(ec.element_to_be_clickable((By.XPATH, adr)))
        elem.click()
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)
    else:
        return elem

# wait before clik
def waitonclick(adr):
    try:
        elem = WebDriverWait(driver, 5).until(
            ec.visibility_of_element_located((By.XPATH, adr)))
        elem.click()
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)

# clear field
def clear(adr):
    driver.find_elements_by_xpath(adr).clear()
    return True

# write text in field
def write(adr, char):
    try:
        elem = driver.find_element_by_xpath(adr)
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)
    else:
        elem.click()
        elem.clear()
        elem.send_keys(char)
        return elem

# check some text on page
def checktxt(txt):
    try:
        assert txt in driver.page_source
    except AssertionError:
        raise TestFailException('Text %s not found' % txt)
    return True

# get list elements
def get_list(adr):
    try:
        elem:list = driver.find_elements_by_xpath(adr)
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)
    else:
        return elem

# check len list
def checklen(lst, ln):
    try:
        assert len(lst) == ln
    except AssertionError:
        raise TestFailException ('The list is not equal to %s' % ln)
    else:
        return True

#################################Classes for tests#######################################
class Registration:

    def __init__(self, fname, lname, mail, pas):
        self.fname = fname
        self.lname = lname
        self.mail = mail
        self.pas = pas

    name = 'registration'

    def start(self):
        try:
            tap(signup)
            time.sleep(1)
            write(firstname, self.fname)
            write(lastname, self.lname)
            write(email, self.mail)
            write(pwd, self.pas)
            tap(submitUP)
            time.sleep(1)
            assert driver.find_element_by_xpath(signup)
            time.sleep(4)
        except TestFailException as e:
            print ('Registration test FAILED' + '\n' + str(e))
        else:
            return True

class Authorization:

    def __init__(self, mail, pas):
        self.mail = mail
        self.pas = pas

    name = 'authorization'

    def start(self):
        try:
            tap(signin)
            write(email, self.mail)
            write(pwd, self.pas)
            tap(submitIN)
            time.sleep(1)
            checktxt('Hi, ')
        except TestFailException as e:
            print ('Authorization test FAILED' + '\n' + str(e))
        else:
            return True

class Store:

    def __init__(self, name, slug, pprice, vcode):
        self.name = name
        self.slug = slug
        self.pprice = pprice
        self.vcode = vcode

    name = 'Store'

    def create(self):
        try:
            tap(wizard)
            checktxt('Give your store a name')
            write(store_name, self.name)
            write(storeSlug, self.slug)
            write(short_desc, 'test short')
            waitonclick(nextstep)
            checktxt('Set up store')
            tap(mainlanguage)
            list_lan = get_list(languages)
            checklen(list_lan, 9)
            list_lan[3].click()
            tap(storeCountry)
            list_countries = get_list(countries)
            assert len(list_countries) > 200
            elem = driver.find_element_by_xpath(frame_country)
            elem2 = elem.find_element_by_xpath(russia)
            actions = ActionChains(driver)
            actions.move_to_element(elem2).click()
            tap(storeCountry)
            write(storeAdress, 'New Arbat Avenue')
            waitonclick(storeSubmitAdress)
            waitonclick(nextstep)
            checktxt('Fill your store with goods')
            tap(addFproduct)
            write(productName, self.name)
            write(short_desc, 'test')
            tap(category)
            tap(category1)
            tap(category2)
            tap(category3)
            write(price, self.pprice)
            write(vendorCode, self.vcode)
            tap(saveProduct)
            checktxt('Fill your store with goods')
            time.sleep(1)
            waitonclick(nextstep)
            #driver.find_element_by_xpath(nextstep).click()
            checktxt('Do you really want to leave this page?')
            tap(closeWizard)
        except TestFailException as e:
            print('Create store test FAILED' + '\n' + str(e))
        except AssertionError:
            print ('List countries changed')
        else:
            return True

    def edit(self):
        try:
            write(store_name, self.name+'edited')
            write(slogan, 'testestest')
            write(storeSlug, self.slug)
            write(short_desc, 'short desc')
            write(long_desc, 'long test')
            tap(save_store)
            time.sleep(1)
            checktxt('Saved!')
        except TestFailException as e:
            print ('Edit store test FAILED' + '\n' + str(e))
        else:
            return True


class User:

    name = 'User'

    def profile(self):
        try:
            tap(user)
            tap(profile)
            tap(gender)
            list_genders = get_list(genders)
            checklen(list_genders, 2)
            list_genders[1].click()
            tap(phone).clear()
            write(phone, '8'+unic)
            tap(save_profile)
            time.sleep(3)
        except TestFailException as e:
            print('User profile update test FAILED' + '\n' + str(e))
        else:
            return True



    def adress_add(self):
        try:
            tap(adresses)
            tap(addAddress)
            tap(userCountry)
            list_countries = get_list(countries)
            assert len(list_countries) > 200
            elem = driver.find_element_by_xpath(usa)
            elem.click()
            write(storeAdress, 'Нью-Йорк, Айова, США')
            waitonclick(storeSubmitAdress)
            write(userPostalcode, 123321)
            tap(saveAddress)
            time.sleep(1)
            checktxt('Address created!')
        except TestFailException as e:
            print('Add shipping address test FAILED' + '\n' + str(e))
        else:
            return True



    def adress_edit(self):
        pass

    def adress_del(self):
        try:
            tap(dellAddress)
            time.sleep(1)
            checktxt('Address deleted!')
        except TestFailException as e:
            print('Delete shipping address test FAILED' + '\n' + str(e))
        else:
            return True

class Checkout:

    def __init__(self, prod):
        self.prod = prod

    name = 'Chekout'

    def buy(self):
        try:
            driver.get(self.prod)
            tap()
        except TestFailException as e:
            print('Buy item test FAILED' + '\n' + str(e))
        else:
            return True
