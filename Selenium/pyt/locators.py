# -*- coding: utf-8 -*-
from datetime import datetime

regmail = 'tester' + datetime.strftime(datetime.now(), "%m.%d_%H%M%S") + '@mail.ru'
testmail = 'tester@test.com'
testdev = "http://stage.stq.cloud/"
testprod = "https://storiqa.com/start"

# Определение локаторов на STORIQA.COM

# ШАПКА САЙТА:
logo = 'a.logo'  # driver.find_element_by_css_selector ('a.logo')	#Кнопка на главную
search = "//input [@class='search']"  # driver.find_element_by_xpath ("//input [@class='search']")	#Поиск по сайту
user = "//div [@class='UserDropdown__avatar___3nspF']"


# АВТОРИЗАЦИЯ: (/signin)
	# регистрация
signup = "//div [text()='Sign Up']"
login = "//label [text()='Username']"
email = "//input [@name='email']"
pwd = "//input [@name='password']"
pwd_conf = "//input [@name='password_confirmation']"  # driver.find_element_by_xpath ("//input [@name='password_confirmation']")	#повтор пароля
submit = "//div[@class='Authorization__signUpButton___285Hx']/button"
	# войти
signin = "//div [text()='Sign In']"
fb = "//button [@class='button facebook']"  # driver.find_element_by_xpath ("//button [@class='button facebook']")	#фейсбук
gg = "" # google
login = "//input [@class='Input__root___BdXh-']"
pwd = "//input [@name='password']"  #driver.find_element_by_xpath ("//input [@name='password']")	#пароль
fg_pwd = u'Forgot?'  # driver.find_element_by_link_text(u'Регистрация')	#забыли
rememberme = "//label[@class='Checkbox__label___2e-fu']"
enter = "//button [@class='Button__container___1Nus9']"

# ГЛАВНАЯ СТРАНИЦА: (/start)
create_store = u'Start selling'  # driver.find_element_by_link_text(u'Создать магазин')	#Создать магазин
# base_tariff = "//div[@class='plan']" #driver.find_element_by_xpath ("//button[text()='Выбрать']")	#Базовый выбрать
base_tariff = "//button[text()='Get Started']"
step2 = "//button[text()='Continue']"
ru = "//label/input[@value='ru']"       #By.XPATH
en = "//label[input/@value='en']"       #By.XPATH
es = "//label/input[@value='es']"       #By.XPATH
cn = "//label/input[@value='cn']"       #By.XPATH
ru_main = "//div/input[@value='ru']"    #By.XPATH
en_main = "//div/input[@value='en']"    #By.XPATH
es_main = "//div/input[@value='es']"    #By.XPATH
cn_main = "//div/input[@value='cn']"    #By.XPATH
name = "//input[@name='store.translations.en.name']"      #By.XPATH
discription = "//textarea[@name='store.translations.en.summary']"
categories = "//div[text()='Enter category name']"
location = "//div[text()='Enter the name of the city']"
phone = "//input[@name='store.phone']"
back = "//button[@class='link back']"
submit2 = "//button[@class='Button__container___1Nus9']"

#РЕДАКТИРОВАНИЕ МАГАЗИНА(/stores/[storeslug]/edit)
catalog = u'Catalog'	 #By.LINK_TEXT
stocks = u'Stocks'	#By.LINK_TEXT
save = "//button[text()='Save']"
remove = "//button[text()='Remove']"

#КАРТОЧКА ТОВАРА: (/product/tovar_name)
product_name = "//div [@class='asside']" #driver.find_element_by_xpath("//div [@class='asside']")	#Название товара
#Цена товара
#Артикул
#Количество
#Доставка в
#Почтой
#Курьером
#Самовывоз
#Итого
cart_add = "//div/button [@class='button']" #driver.find_element_by_xpath("//div/button [@class='button']")	#В корзину
#Купить в 2 клика
#Описание
#Доставка
#Отзывы
#Гарантии
#Возврат



