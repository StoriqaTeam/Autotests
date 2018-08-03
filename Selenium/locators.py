# -*- coding: utf-8 -*-
from datetime import datetime
import re

# Служебные переменные
unic = datetime.strftime(datetime.now(), "%m%d%H%M%S") # Уникальная переменная
regmail = 'tester' + unic + '@test.test' # Адрес электронной почты для регистраций
regname = 'test' + unic # Имена для магазинов и товаров
testmail = 'tester@storiqa.com' # Логин тестового пользователя
testdev = "https://nightly.stq.cloud/" # Адрес тестового стенда
testprod = "https://storiqa.com/start" # Адрес продакшена

''' Определение локаторов на STORIQA.COM '''

# ШАПКА САЙТА:
logo = "//a[@data-test='logoLink']"  # Логотип Сторика
select = "//div[@class='Select__icon___QSozW']" # Выбор продукты или магазины
search_query = "//input[@data-test='searchInput']" # Поле поиска по сайту
search = "//div[button/@data-test='searchButton']" # Кнопка поиска
user = "//div[@data-test='userDropdownButton']" # Открыть меню юзера
currency = "//div [@data-test='headerСurrenciesSelect']" # Выбор валюты
currencies = "//div[@data-test='']"
lang = "//div[@data-test='headerLanguagesSelect']" # Выбор языка
langs = "//div[@data-test='']"
helpp = "//a [@href='']" # Помощь
wizard = "//a [@href='/manage/wizard']" # Создание магазина
cart = "//a[@data-test='header-cart-button']" # Корзина

# АВТОРИЗАЦИЯ: (/signin /signup)
signup = "//div [@data-test='headerSignUpButton']" # Регистрация
firstname = "//input [@data-test='firstName']" # Имя
lastname = "//input [@data-test='lastName']" # Фамилия
email = "//input [@data-test='email']" # Електронная почта
pwd = "//input [@data-test='password']" # Пароль
submitUP = "//button[@data-test='signUpButton']" # Кнопка подтвердить регитсрацию
signin = "//div [@data-test='headerSignInButton']"# Авторизация
fb = "//a [@data-test='authFacebookButton']"  # Авторизация через фейсбук
gg = "//a[@data-test='authGoogleButton']"  # Афторизация через google
fg_pwd = ''  # Забыли пароль
submitIN = "//button[@data-test='signInButton']" # Кнопка подтвердить авторизацию

# МЕНЮ ЮЗЕРА
messages = "" #
profile = "//a[@data-test='header-user-menu-profileLink']" # Профиль пользователя
history = "" #
myshops = "" #
logout = "//a[@data-test='header-user-menu-logoutLink']" # Логаут

# ПРОФИЛЬ ПОЛЬЗОВАТЕЛЯ
# Персональные данные
userFname = "//input[@data-test='firstName']" # Имя
userLname = "//input[@data-test='lastName']" # Фамилия
gender = "//div[@data-test='profileGenderSelect']" # Пол
genders = "//div[@data-test='profileGenderSelect_items']" # Все элементы списка пол
birthdate = {
    'year' : "//class[@data-test='yearSelectBirthdateProfile']",
    'month' : "//class[@data-test='monthSelectBirthdateProfile']",
    'day' : "//class[@data-test='daySelectBirthdateProfile']"
            } # Дата рождения
years = "//div[@data-test='']"
months = "//div[@data-test='']"
days = "//div[@data-test='']"
phone = "//input[@data-test='phone']" # Телефон
save_profile = "//button[@data-test='saveButton']" # Сохранить
# Адреса доставки
adresses = "//a[@href='/profile/shipping-addresses']" # Выбор меню адресов доставки
userCountry = "//div[@data-test='AddressFormSelect']" # Страна пользователя
countries = "//div[@data-test='AddressFormSelect_items']" # Список стран
userAdress = "//input[@data-test='autocompleteAddress']" # Адрес пользователя
userSuite = "//input[@data-test='streetNumber']" # Сокращение
userStreet = "//input[@data-test='route']" # Улица
userCity = "//input[@data-test='locality']" # Город
userRegion = "//input[@data-test='administrativeAreaLevel2']" # Регион
userArea = "//input[@data-test='administrativeAreaLevel1']" # Зона
userPostalcode = "//input[@data-test='postalCode']" # Почтовый код
priority = "//input[@data-test='priority']" # Приоритетность адреса
addAddress = "//button[@data-test='addShippingAddressButton']" # Кнопка добавить адрес
saveAddress = "//button[@data-test='saveShippingAddressButton']" # Кнопка сохранить адрес
dellAddress = "//button[@data-test='deleteShippingAddressButton']" # Удалить адрес

# Безопасность
# KYC

# СОЗДАНИЕ МАГАЗИНА(/manage/wizard)
# wizard
first_step = "//div[contains(text(), 'Give your store a name')]" # Определение первого шага
store_name = "//input[@data-test='name']" # Имя магазина
storeSlug = "//input[@data-test='slug']" # Слуг
short_desc = "//textarea[@data-test='shortDescription']" # Краткое описание
nextstep = "//button[@data-test='wizardBackButton']" # Кнопка некст степ
two_step = "//div[contains(text(), 'Set up store')]" # Определение второго шага
mainlanguage = "//div[@data-test='wizardLanguagesSelect']" # Язык магазинга
languages = "//div[@data-test='wizardLanguagesSelect_items']" # Список языков
ru = "//div[@data-test='RU']" # Русский язык
storeCountry = "//div[@data-test='AddressFormSelect']" # Страна магазина
frame_country = "//div[@class='Select__itemsWrap___2oJi_']" # Окно со списком
countries # Список стран
russia = "//div/div/div[@id='RU']" # Россия
usa = "//div/div/div[@id='US'][@data-test='AddressFormSelect_items']" # США
storeAdress = "//input[@data-test='autocompleteAddress']"  # Адрес магазина
storeSubmitAdress = "//div[@class='AddressForm__items___3Wr7L']"
storeSuite = "//div[@data-test='streetNumber']" # Сокращение
storeStreet = "//div[@data-test='route']" # Улица
storeCity = "//div[@data-test='locality']" # Город
storeRegion = "//div[@data-test='administrativeAreaLevel2']" # Регион \ штат
storeArea = "//div[@data-test='administrativeAreaLevel1']" # Зона
storePostalcode = "//div[@data-test='postalCode']" # Почтовый индекс
three_step = "//div[contains(text(), 'Fill your store with goods')]" # Определение третьего шага
addFproduct = "//button[@data-test='wizardUploaderProductFotoFirst']" # Кнопка добавить продукт
productName =  "//input [@data-test='name']" #
short_desc
category = "//div[@data-test='categorySelector']"
category1 = "//div[@data-test='categoryItem_36']"
category2 = "//div[@data-test='categoryItem_37']"
category3 = "//div[@data-test='categoryItem_38']"
vendorCode = "//input[@data-test='vendorCode']"
price = "//input[@data-test='price']"
saveProduct = "//button[@data-test='wizardSaveProductButton']"
closeWizard =  "//button[@data-test='closeWizard']"
continueWizard =  "//button[@data-test='continueWizard']"
# РЕДАКТИРОВАНИЕ МАГАЗИНА
# settings
settings = ""
store_name
storeLang = "//div[@data-test='storeLangSelect']"
slogan = "//input[@data-test='slogan']" #
storeSlug
short_desc
long_desc = "//textarea[@data-test='longDescription']" #
save_store = "//button[@class='SpinnerButton__container___1_OzA']" #
goods = ""

# СОЗДАНИЕ ТОВАРА: (/product/tovar_name)
seo_title = ""
seo_desc = ""

full_desc = "" #



# КОРЗИНА
plusquantityincart = "//button[data-test='cartProductIncreaseButton']" #
minusquantityincart = "//button[data-test='cartProductsDecreaseButton']" #
stq = ""
btc = ""
eth = ""
checkout = "//button[data-test='checkoutNext']" # Кнопка чекаут
del_product = "//button[@data-test='cartProductDeleteButton']" # Удалить товар из корзины
reciever_name = "//input[@data-test='receiverName']" # Имя получателя
select_address = "//div[@data-test='selectExistingAddress']" # Выбрать из списка адресов
address1 = "//div [@data-test='selectExistingAddress_items'][@id='New Arbat Avenue']" # Один и списка адресов
nextSubmit = "//button [@data-test='checkoutNext']" # Следующий шаг
replaceAddress = "//button[@data-test='changeAddress']" # Изменить адрес


#ПРОДУКТЫ И МАГАЗИНЫ
product = "//a [@data-test='668']" # Первый попавшийся продукт
sproduct = "https://nightly.stq.cloud/store/136/products/529" # Конкретный продукт для тестов
addprod = "//button[@data-test='product-addToCart']" # Добавить продукт в корзину.