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
# driver = webdriver.Chrome()
driver.maximize_window()
driver.get(url)
time.sleep(4)
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
            ec.element_to_be_clickable((By.XPATH, adr)))
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
        #elem = driver.find_element_by_xpath(adr)
        elem =WebDriverWait(driver, 5).until(
            ec.visibility_of_element_located((By.XPATH, adr)))
        elem.click()
        elem.clear()
        elem.send_keys(char)
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)
    return elem

# check element present on page
def checkelem(adr):
    try:
        WebDriverWait(driver, 10).until(
            ec.presence_of_element_located((By.XPATH, adr)))
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)

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
        elem = driver.find_elements_by_xpath(adr)
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

    def positive(self):
        try:
            waitonclick(signup)
            #time.sleep(1)
            write(firstname, self.fname)
            write(lastname, self.lname)
            write(email, self.mail)
            write(pwd, self.pas)
            tap(submitUP)
            checkelem(success)
            tap(closeAlert)
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
            waitonclick(signin)
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
            waitonclick(startSell)
            waitonclick(wizard)
            # time.sleep(2)
            # checktxt('Give your store a name')
            checkelem(first_step)
            write(store_name, self.name)
            write(storeSlug, self.slug)
            time.sleep(1)
            write(short_desc, 'test short')
            waitonclick(nextstep)
            checkelem(two_step)
            # WebDriverWait(driver, 5).until(
            #     ec.visibility_of_element_located((By.XPATH, two_step)))
            tap(mainlanguage)
            list_lan = get_list(languages)
            checklen(list_lan, 1)
            list_lan[0].click()
            tap(country)
            list_countries = get_list(countries)
            assert len(list_countries) == 249
            elem = driver.find_element_by_xpath(russia)
            elem.click()
            write(storeAdress, 'New Arbat Avenue')
            waitonclick(storeSubmitAdress)
            waitonclick(nextstep)
            checkelem(three_step)
            # WebDriverWait(driver, 5).until(
            #     ec.visibility_of_element_located((By.XPATH, three_step)))
            tap(addNproduct)
            write(productName, self.name)
            write(short_desc, 'test')
            tap(category)
            tap(category1)
            tap(category2)
            tap(category3)
            write(price, self.pprice)
            write(vendorCode, self.vcode)
            tap(currency)
            tap(stq)
            write(cashback, 6)
            write(quantity, 6)
            tap(saveProduct)
            WebDriverWait(driver, 5).until(
                ec.visibility_of_element_located((By.XPATH, three_step)))
            #tap(deleteProduct)
            #tap(yesDeleteProduct)
            waitonclick(nextstep)
            checktxt('Do you really want to leave this page?')
            tap(continueWizard)
        except TestFailException as e:
            print('Create store test FAILED' + '\n' + str(e))
        except AssertionError:
            print ('List countries changed')
        else:
            return True

    def edit(self):
        try:
            waitonclick(user)
            tap(myshops)
            write(store_name, self.name+'edited')
            write(slogan, 'testestest')
            write(short_desc, 'short desc')
            write(long_desc, 'long test')
            tap(save_store)
            checkelem(success)
            waitonclick(storages)
            waitonclick(edit_storage)
            write(storage_name, 'teststorage')
            tap(country)
            list_countries = get_list(countries)
            assert len(list_countries) == 250
            elem = driver.find_element_by_xpath(russia)
            elem.click()
            write(storeAdress, 'New Arbat Avenue')
            waitonclick(storeSubmitAdress)
            tap(save_storage)
            checkelem(success)
            waitonclick(closeAlert)
        except TestFailException as e:
            print ('Edit store test FAILED' + '\n' + str(e))
        else:
            return True

    def goods(self):
        waitonclick(goods)
        tap(add_item)
        write(productName, self.name)
        write(seo_title, 'test')
        write(seo_desc, 'test seo')
        write(short_desc, 'short')
        write(plong_desc, 'long')
        tap(category)
        tap(category1)
        tap(category2)
        tap(category3)



class User:

    name = 'User'

    def profile(self):
        try:
            waitonclick(user)
            tap(profile)
            tap(gender)
            list_genders = get_list(genders)
            checklen(list_genders, 2)
            list_genders[1].click()
            tap(phone).clear()
            write(phone, '8'+unic)
            tap(save_profile)
            checkelem(success)
            tap(closeAlert)
        except TestFailException as e:
            print('User profile update test FAILED' + '\n' + str(e))
        else:
            return True

    def adress_add(self):
        try:
            waitonclick(adresses)
            tap(country)
            list_countries = get_list(countries)
            assert len(list_countries) > 200
            elem = driver.find_element_by_xpath(usa)
            elem.click()
            write(storeAdress, 'Нью-Йорк, Айова, США')
            waitonclick(storeSubmitAdress)
            write(postalcode, 123321)
            tap(saveAddress)
            checkelem(success)
            tap(closeAlert)
        except TestFailException as e:
            print('Add shipping address test FAILED' + '\n' + str(e))
        else:
            return True

    def adress_del(self):
        try:
            tap(dellAddress)
            checkelem(success)
            tap(closeAlert)
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
            write(search_query, 'motorcycle')
            tap(search)
            waitonclick(product)

        except TestFailException as e:
            print('Buy item test FAILED' + '\n' + str(e))
        else:
            return True
