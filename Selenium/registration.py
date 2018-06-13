#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time
from locators import *
import main as m
from selenium.common.exceptions import NoSuchElementException
from selenium import webdriver


# click on object
def tap(adr):
    try:
        m.driver.find_element_by_xpath(adr).click()
    except NoSuchElementException as e:
        print(e)
        return False
    return True


# write text in field
def write(adr, char):
    try:
        elem = m.driver.find_element_by_xpath(adr)
    except NoSuchElementException as e:
        print(e)
        return False
    elem.click()
    elem.send_keys(char)
    return True


class Registration:

    def __init__(self, fname, lname, mail, pas):
        self.fname = fname
        self.lname = lname
        self.mail = mail
        self.pas = pas

    test_name = 'registration'

    def reg(self):
        tap(signup)
        time.sleep(1)
        write(firstname, self.fname)
        write(lastname, self.lname)
        write(email, self.mail)
        write(pwd, self.pas)
        tap(submitUP)
        time.sleep(1)
        assert m.driver.find_element_by_xpath(signup)


if __name__ == "__main__":
    driver = webdriver.Chrome()
    driver.get(testdev)
    driver.implicitly_wait(5)
    assert "Storiqa" in driver.title

    test_registration = Registration('tester', 'testoviy', regmail, 'qwe123QWE')
    test_registration.reg