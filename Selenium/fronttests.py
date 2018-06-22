#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import os
from locators import *
from selenium.common.exceptions import NoSuchElementException
from selenium import webdriver
from selenium.webdriver.common.action_chains import ActionChains
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.support.ui import WebDriverWait
from selenium.webdriver.support import expected_conditions as EC

if os.getenv('SELENIUM_URL'):
    url = os.environ['SELENIUM_URL']
else: url = testdev

driver = webdriver.Chrome()
driver.maximize_window()
driver.get(url)
driver.implicitly_wait(2)
assert "Storiqa" in driver.title

########################################Interface#########################################
# Exception for failed test
class TestFailException(Exception):
    pass

# click on object
def tap(adr):
    try:
        elem = driver.find_element_by_xpath(adr)
        elem.click()
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)
    else:
        return elem

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

    test_name = 'registration test'

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

    test_name = 'authorization'

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

    def __init__(self, name, slug, price):
        self.name = name
        self.slug = slug
        self.price = price

    def create(self):
        try:
            tap(wizard)
            checktxt('Give your store a name')
            write(store_name, self.name)
            write(storeSlug, self.slug)
            write(short_desc, 'test short')
            tap(nextstep)
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
            time.sleep(1)
            driver.find_element_by_xpath(storeSubmitAdress).click()
            tap(nextstep)
        except TestFailException as e:
            print('Create store test FAILED' + '\n' + str(e))
        except AssertionError:
            print ('List countries changed')
        else:
            return True



class User:

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
            time.sleep(4)
        except TestFailException as e:
            print('User profile update test FAILED' + '\n' + str(e))
        else:
            return True



    def adress_add(self):
        tap(adresses)
        tap(addAdress)



    def adress_edit(self):
        pass

    def adress_del(self):
        pass