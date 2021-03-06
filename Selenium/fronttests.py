#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
import os
from locators import *
from selenium.common.exceptions import NoSuchElementException
from selenium import webdriver
from selenium.common.exceptions import TimeoutException
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
     desired_capabilities=DesiredCapabilities.CHROME)
# driver = webdriver.Chrome()
# driver.maximize_window()
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
        elem = WebDriverWait(driver, 5).until(
            ec.visibility_of_element_located((By.XPATH, adr)))
        elem.click()
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)
    except TimeoutException:
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
    except TimeoutException:
        raise TestFailException('Object "%s" not found' % adr)

# clear field
def clear(adr):
    driver.find_elements_by_xpath(adr).clear()
    return True

# write text in field
def write(adr, char):
    try:
        elem =WebDriverWait(driver, 5).until(
            ec.visibility_of_element_located((By.XPATH, adr)))
        elem.click()
        elem.clear()
        elem.send_keys(char)
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)
    except TimeoutException:
        raise TestFailException('Object "%s" not found' % adr)
    return elem

# check element present on page
def checkelem(adr):
    try:
        WebDriverWait(driver, 10).until(
            ec.presence_of_element_located((By.XPATH, adr)))
    except NoSuchElementException:
        raise TestFailException('Object "%s" not found' % adr)
    except TimeoutException:
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

# move cursor on element and click
def cursor_click(adr):
    try:
        elem = driver.find_element_by_xpath(adr)
        ActionChains(driver).move_to_element(elem).perform()
        elem.click()
    except NoSuchElementException:
        print('Element %s not found' % adr)

def check_address_correct():
    try:
        elem = driver.find_element_by_xpath(adress)
        a = elem.get_attribute('value')
        assert a == test_address['address']
        elem = driver.find_element_by_xpath(suite)
        a = elem.get_attribute('value')
        assert a == test_address['suite']
        elem = driver.find_element_by_xpath(street)
        a = elem.get_attribute('value')
        assert a == test_address['street']
        elem = driver.find_element_by_xpath(city)
        a = elem.get_attribute('value')
        assert a == test_address['locality']
        elem = driver.find_element_by_xpath(region)
        a = elem.get_attribute('value')
        assert a == test_address['region']
        elem = driver.find_element_by_xpath(area)
        a = elem.get_attribute('value')
        assert a == test_address['area']
        elem = driver.find_element_by_xpath(postalcode)
        a = elem.get_attribute('value')
        assert a == test_address['pcode']
    except AssertionError as err:
        print('Address incorrect'+ str(err))
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
            write(firstname, self.fname)
            write(lastname, self.lname)
            write(email, self.mail)
            write(pwd, self.pas)
            cursor_click(terms)
            cursor_click(privacy)
            # elem = driver.find_element_by_xpath(terms)
            # ActionChains(driver).move_to_element(elem).perform()
            # elem.click()
            # elem = driver.find_element_by_xpath(privacy)
            # ActionChains(driver).move_to_element(elem).perform()
            # elem.click()
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
        #self.address = address

    name = 'Store'

    def create(self):
        try:
            waitonclick(startSell)
            waitonclick(wizard)
            checkelem(first_step)
            write(store_name, self.name)
            write(storeSlug, self.slug)
            time.sleep(1)
            write(short_desc, 'test short')
            waitonclick(nextstep)
            checkelem(two_step)
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
            tap(save_firstProduct)
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
            tap(closeAlert)
            waitonclick(storages)
            waitonclick(edit_storage)
            write(storage_name, 'teststorage')
            check_address_correct()
            tap(save_storage)
            checkelem(success)
            waitonclick(closeAlert)
        except TestFailException as e:
            print ('Edit store test FAILED' + '\n' + str(e))
        else:
            return True

    def goods(self):
        try:
            time.sleep(0.5)
            waitonclick(user)
            tap(myshops)
            waitonclick(goods)
            tap(add_item)
            tap(save_product)
            time.sleep(0.5)
            checktxt('Name is required')
            checktxt('Short description is required')
            checktxt('Long description is required')
            checktxt('Vendor code is required')
            checktxt('Category is required')
            checktxt('Price is required')
            tap(category)
            tap(category1)
            tap(category2)
            tap(category3)
            write(productName, self.name)
            write(seo_title, 'test')
            write(seo_desc, 'test seo')
            write(short_desc, 'short')
            write(plong_desc, 'long')
            write(vendorCode, self.vcode)
            write(v_price, self.pprice)
            write(v_cashback, 5)
            write(v_discount, 10)
            write(preOrder_days, 5)
            cursor_click(preOrder_check)
            tap(attr_select)
            tap(attr_item)
            tap(attr_add)
            tap(save_product)
            checkelem(success)
            tap(closeAlert)
            tap(goods)
            waitonclick(delete_product)
            checkelem(success)
            tap(closeAlert)
        except TestFailException as e:
            print('Test Edit Goods FAILED' + '\n' + str(e))
        else:
            return True


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
            write(phone, '+7'+unic)
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
            tap(logo)
            waitonclick(allCat)
            waitonclick()


        except TestFailException as e:
            print('Buy item test FAILED' + '\n' + str(e))
        else:
            return True
