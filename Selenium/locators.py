# -*- coding: utf-8 -*-
from datetime import datetime

unic = datetime.strftime(datetime.now(), "%m%d%H%M%S")
regmail = 'tester' + unic  + '@mail.ru'
testmail = 'tester@test.test'
testdev = "https://stage.stq.cloud/"
testprod = "https://storiqa.com/start"

# Определение локаторов на STORIQA.COM

# ШАПКА САЙТА:
logo = 'a.logo'  # driver.find_element_by_css_selector ('a.logo')	#Кнопка на главную
search = "//input [@class='search']"  # driver.find_element_by_xpath ("//input [@class='search']")	#Поиск по сайту
user = "//div [@class='UserDropdown__avatar___3nspF']"


# АВТОРИЗАЦИЯ: (/signin /signup)
signup = "//div [text()='Sign Up']"
login = "//input [@data-test='username']"
email = "//input [@data-test='email']"
pwd = "//input [@data-test='password']"
pwd_conf = "//input [@name='password_confirmation']"  # driver.find_element_by_xpath ("//input [@name='password_confirmation']")	#повтор пароля
submitUP = "//button[@data-test='signUpButton']"
signin = "//div [text()='Sign In']"
fb = "//button [@class='button facebook']"  # driver.find_element_by_xpath ("//button [@class='button facebook']")	#фейсбук
gg = "" # google
fg_pwd = u'Forgot?'  # driver.find_element_by_link_text(u'Регистрация')	#забыли
submitIN = "//button[@data-test='signInButton']"


# ГЛАВНАЯ СТРАНИЦА: (/start)
start_selling = "//a [@data-test='headerStartSellingButton']"


#СЩХДАНИЕ МАГАЗИНА(/manage/store/new)
store_name = "//input[@data-test='name']"
slogan = "//input[@data-test='slogan']"
slug = "//input[@data-test='slug']"
short_desc = "//textarea[@data-test='shortDescription']"
long_desc = "//textarea[@data-test='longDescription']"
save_store = "//button[@class='SpinnerButton__container___1_OzA']"

#СОЗДАНИЕ ТОВАРА: (/product/tovar_name)
product_name = "//input [@data-test='name']"
seo_title = ""
seo_desc = ""
short_desc
full_desc = ""
category = ""
save_product = ""

#Самовывоз
#Итого
cart_add = "//div/button [@class='button']"
#Купить в 2 клика
#Описание
#Доставка
#Отзывы
#Гарантии
#Возврат



