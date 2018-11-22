# -*- coding: utf-8 -*-
from datetime import datetime
import re

# Служебные переменные
unic = datetime.strftime(datetime.now(), "%m%d%H%M%S") # Уникальная переменная
regmail = 'tester' + unic + '@test.test' # Адрес электронной почты для регистраций
regname = 'test' + unic # Имена для магазинов и товаров
testmail = 'autotester@storiqa.com' # Логин тестового пользователя
testdev = "https://storiqateam:s3cur3passw0rd@nightly.stq.cloud/auth" # Адрес тестового стенда
testprod = "https://storiqa:whenLambo%3F@stage.stq.cloud/auth" # Адрес предрелизного стенда
test_address = {"country" : "Russian Federation",
                "address" : "New Arbat Avenue",
                "suite" : "33/5",
                "street" : "New Arbat Avenue",
                "locality" : "Moskva",
                "region" : "Moskva",
                "area" : "moscow",
                "pcode" : '123321'} # Данные формы адреса

''' Определение локаторов на STORIQA.COM '''

# ШАПКА САЙТА:
logo = "//a[@data-test='logoLink']"  # Логотип Сторика
select = "//div[@class='Select__icon___QSozW']" # Выбор продукты или магазины
search_query = "//input[@data-test='searchInput']" # Поле поиска по сайту
search = "//div[button/@data-test='searchButton']" # Кнопка поиска
user = "//div[@data-test='userDropdownButton']" # Открыть меню юзера
headercurrency = "//div [@data-test='headerСurrenciesSelect']" # Выбор валюты
currencies = "//div[@data-test='headerСurrenciesSelect_items']" # Список валют
lang = "//div[@data-test='headerLanguagesSelect']" # Выбор языка
langs = "//div[@data-test='headerLanguagesSelect_items']" # Список языков
helpp = "//a [@href='']" # Помощь
startSell = "//a [@href='/start-selling']"
wizard = "//div[@data-test='startSelling']" # Создание магазина
cart = "//a[@data-test='header-cart-button']" # Корзина

# ФИЛЬТРЫ
allCat = "//a[@data-test='allCategoriesLink']" #
cycling = "//div[@data-test='categoryLevelTwoButton'][contains(text(), 'Cycling')]"
Bicycle = ""

# АВТОРИЗАЦИЯ: (/signin /signup)
signup = "//div [@data-test='headerSignUpButton']" # Регистрация
firstname = "//input [@data-test='firstName']" # Имя
lastname = "//input [@data-test='lastName']" # Фамилия
email = "//input [@data-test='email']" # Електронная почта
pwd = "//input [@data-test='password']" # Пароль
terms = "//div[input/@data-test='terms']" #
privacy = "//div[input/@data-test='privacy']" #
submitUP = "//button[@data-test='signUpButton']" # Кнопка подтвердить регитсрацию
signin = "//div [@data-test='headerSignInButton']"# Авторизация
fb = "//a [@data-test='authFacebookButton']"  # Авторизация через фейсбук
gg = "//a[@data-test='authGoogleButton']"  # Афторизация через google
fg_pwd = ''  # Забыли пароль
submitIN = "//button[@data-test='signInButton']" # Кнопка подтвердить авторизацию

# МЕНЮ ЮЗЕРА
profile = "//a[@data-test='header-user-menu-profileLink']" # Профиль пользователя
orders = "" #
myshops = "//a[@data-test='header-user-menu-myShops']" #
logout = "//a[@data-test='header-user-menu-logoutLink']" # Логаут

# ПРОФИЛЬ ПОЛЬЗОВАТЕЛЯ
# Персональные данные
userFname = "//input[@data-test='firstName']" # Имя
userLname = "//input[@data-test='lastName']" # Фамилия
gender = "//div[@data-test='profileGenderSelect']" # Пол
genders = "//div[@data-test='profileGenderSelect_item']" # Все элементы списка пол
birthdate = {
    'year' : "//class[@data-test='yearSelectBirthdateProfile']",
    'month' : "//class[@data-test='monthSelectBirthdateProfile']",
    'day' : "//class[@data-test='daySelectBirthdateProfile']"
            } # Дата рождения
phone = "//input[@data-test='phone']" # Телефон
save_profile = "//button[@data-test='saveButton']" # Сохранить
# Адреса доставки
adresses = "//a[@href='/profile/shipping-addresses']" # Выбор меню адресов доставки
country = "//div[@data-test='AddressFormSelect']" # Страна пользователя
countries = "//div[@data-test='AddressFormSelect_item']" # Список стран
adress = "//input[@data-test='autocompleteAddress']" # Адрес пользователя
suite = "//input[@data-test='streetNumber']" # Сокращение
street = "//input[@data-test='route']" # Улица
city = "//input[@data-test='locality']" # Город
region = "//input[@data-test='administrativeAreaLevel2']" # Регион
area = "//input[@data-test='administrativeAreaLevel1']" # Зона
postalcode = "//input[@data-test='postalCode']" # Почтовый код
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
languages = "//div[@data-test='wizardLanguagesSelect_item']" # Список языков
ru = "//div[@data-test='RU']" # Русский язык
country
countries
adress
suite
street              # ПЕРЕМЕННЫЕ ОПИСАНЫ В РАЗДЕЛЕ АДРЕСОВ ДОСТАВКИ
city
region
area
postalcode
russia = "//div/div/div[@id='RUS'][@data-test='AddressFormSelect_item']" # Россия
usa = "//div/div/div[@id='USA'][@data-test='AddressFormSelect_item']" # США
storeAdress = "//input[@data-test='autocompleteAddress']"  # Адрес магазина
storeSubmitAdress = "//div[@class='AddressForm__items___3Wr7L']"
# storeSuite = "//div[@data-test='streetNumber']" # Сокращение
# storeStreet = "//div[@data-test='route']" # Улица
# storeCity = "//div[@data-test='locality']" # Город
# storeRegion = "//div[@data-test='administrativeAreaLevel2']" # Регион \ штат
# storeArea = "//div[@data-test='administrativeAreaLevel1']" # Зона
# storePostalcode = "//div[@data-test='postalCode']" # Почтовый индекс
three_step = "//div[contains(text(), 'Fill your store with goods')]" # Определение третьего шага
addFproduct = "//button[@data-test='wizardUploaderProductFotoFirst']" # Кнопка добавить первый продукт
addNproduct = "//div[@data-test='wizardUploaderProductFoto']" # Добавить еще один продукт
productName =  "//input [@data-test='name']" # Название продукта
short_desc
category = "//div[@data-test='categorySelector']"
category1 = "//div[@data-test='categoryItem_1']"
category2 = "//div[@data-test='categoryItem_178']"
category3 = "//div[@data-test='categoryItem_179']"
vendorCode = "//input[@data-test='vendorCode']"
price = "//input[@data-test='price']"
currency = "//div[@class='Select__container___3Ai0z Select__forForm___3OBqy']" # валюта
stq = "//div[@id='1'][@data-test='undefined_item']" # Список валют
cashback = "//input[@data-test='cashback']" # Кешбэк
discount = "//input[@data-test='variantDiscountInput']" #
quantity = "//input[@data-test='quantity']" # Количество товара
save_firstProduct = "//button[@data-test='wizardSaveProductButton']" # Сохранить товар в визарде
editProduct = "//span[@data-test='wizardEditProductButton']" # Редактировать товар в визарде
deleteProduct = "//span[@data-test='wizardDeleteProductButton']" # Удалить второй товар в визарде
yesDeleteProduct = "//button[@data-test='wizardDeleteProductButton']" # Подтвердить удаление
noDeleteProduct = "//button[@data-test='wizardDeleteProductCancelButton']" # Отменить удаление
closeWizard =  "//button[@data-test='closeWizard']" # Кнопка закрыть визард
continueWizard =  "//button[@data-test='continueWizard']" # Продолжить создание продукта
# РЕДАКТИРОВАНИЕ МАГАЗИНА
# settings
settings = "//div[@data-test='store-menu-settings']"
store_name
storeLang = "//div[@data-test='storeLangSelect']"
slogan = "//input[@data-test='slogan']" #
storeSlug
short_desc
long_desc = "//textarea[@data-test='longDescription']" #
save_store = "//button[@class='SpinnerButton__container___1_OzA']" #
goods = "//div[@data-test='store-menu-goods']"
# storages
storages = "//div[@data-test='store-menu-storages']"
storage_name = "//input[@data-test='name']"
adr_elem_list = [
country,
countries,
adress,
suite,
street,           # ПЕРЕМЕННЫЕ ОПИСАНЫ В РАЗДЕЛЕ АДРЕСОВ ДОСТАВКИ
city,
region,
area,
postalcode] # Список полей формы адреса
save_storage = "//button[@data-test='saveStorageButton']"
cansel_storage = "//button[@data-test='cancelSaveStorageButton']"
edit_storage = "//button[@data-test='editStorageDataButton']"
delete_storage = "//button[@data-test='deleteStorageButton']"
# goods
add_item = "//button[@data-test='addProductButton']"
seo_title = "//input[@data-test='seoTitle']"
seo_desc = "//textarea[@data-test='seoDescription']"
plong_desc = "//textarea[@data-test='longDescription']" #
edit_variant = "//div[@data-test='toggleOpenVariantButton']"
save_product = "//button[@data-test='saveProductButton']"
add_variant = "//buttom[@data-test='addVariantButton']"
vendorCode
v_price = "//input[@data-test='variantPriceInputInput']"
v_cashback = "//input[@data-test='variantCashbackInput']"
v_discount = "//input[@data-test='variantDiscountInput']"
preOrder_check = "//div[input/@data-test='preOrderCheckbox']"
preOrder_days = "//input[@data-test='variantPreOrderDaysInput']"
attr_select = "//div[@data-test='customAttributtesSelect']"
attr_item = "//div[@data-test='customAttributtesSelect_item'][contains(text(), 'Material')]"
attr_add = "//button[@data-test='customAttributtesAddButton']"
noLocalShip = "//div[@data-test='withoutLocalShipping']" #
localFixedAll = "//div[@data-test='localShippingFixPrice']" #
pickup = "//div[input/@data-test='localPickupCheckbox']"
noInterShip = "//div[@data-test='interShippingWithout']" #
interFixedAll = "" #
save_new_variant = "//button[@data-test='variantsProductSaveButton']"
cancel_new_variant = "//button[@data-test='cancelNewVariantButton']"
delete_product = "//button[@data-test='deleteProductButton']"
# contacts
contacts = "//div[@data-test='store-menu-contacts']"
store_email = "//input[@data-test='email']"
store_phone = "//input[@data-test='phone']"
store_fb = "//input[@data-test='facebookUrl']"
store_inst = "//input[@data-test='instagramUrl']"
store_twit = "//input[@data-test='twitterUrl']"
adr_elem_list # Список полей формы адреса


# АЛЕРТЫ
closeAlert = "//button[@name='alertCloseButton']"
success = "//div/div[contains(text(), 'Success!')]"

# КОРЗИНА
plusquantityincart = "//button[data-test='cartProductIncreaseButton']" #
minusquantityincart = "//button[data-test='cartProductsDecreaseButton']" #
checkout = "//button[data-test='checkoutNext']" # Кнопка чекаут
del_product = "//button[@data-test='cartProductDeleteButton']" # Удалить товар из корзины
reciever_name = "//input[@data-test='receiverName']" # Имя получателя
reciever_phone = "//input[@data-test='receiverPhone']" # Телефон получателя
select_address = "//div[@data-test='selectExistingAddress']" # Выбрать из списка адресов
address1 = "//div [@data-test='selectExistingAddress_items'][@id='New Arbat Avenue']" # Один и списка адресов
nextSubmit = "//button [@data-test='checkoutNext']" # Следующий шаг
replaceAddress = "//button[@data-test='changeAddress']" # Изменить адрес


#ПРОДУКТЫ И МАГАЗИНЫ
product = "//a [@data-test='951']" # Тестовый продукт
sproduct = "https://nightly.stq.cloud/store/136/products/529" # Конкретный продукт для тестов
atrSize44 = "//button[@data-test='productSize44" #
atrMaterial = "" #
atrColour = "" #
addprod = "//button[@data-test='product-addToCart']" # Добавить продукт в корзину.