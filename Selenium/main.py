# -*- coding: utf-8 -*-
from locators import *
import fronttests as r
import time

passed_test_count = 0
failed_test_count = 0

# Tests with parametrs
registration = r.Registration('Tester', 'Testoviy', regmail, 'qwe123QWE')
authorization = r.Authorization(testmail, 'qwe123QWE')
user_profile = r.User()
store = r.Store(regname, regname, 333, unic)
buy_item = r.Checkout(product)

# test_suite = [authorization.start(), store.edit(), store.goods()]
test_suite = [registration.positive(), authorization.start(), user_profile.profile(),
              user_profile.adress_add(), user_profile.adress_del(),
              store.edit(), store.goods()]

if __name__ == "__main__":

    for i in test_suite:
        if i is True:
            passed_test_count += 1
        else:
            failed_test_count += 1
    print ('Test finished with %s PASSED and %s FAILED' % (passed_test_count, failed_test_count))
    if failed_test_count > 0:
        raise r.TestFailException()


    #input('END')