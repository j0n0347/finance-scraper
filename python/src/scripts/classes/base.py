from enum import IntEnum
from enum import StrEnum
from selenium import webdriver
from selenium.webdriver import FirefoxOptions
from selenium import webdriver
from selenium.webdriver import FirefoxOptions
from selenium.webdriver.common.keys import Keys
from selenium.webdriver.remote.webdriver import WebDriver
from selenium.webdriver.remote.webelement import WebElement
from selenium.webdriver.common.by import By
from selenium.common.exceptions import NoSuchElementException
import time
import os
from dotenv import load_dotenv



class Year(IntEnum):
    YEAR_2001 = 2001
    YEAR_2002 = 2002
    YEAR_2003 = 2003
    YEAR_2004 = 2004
    YEAR_2005 = 2005
    YEAR_2006 = 2006
    YEAR_2007 = 2007
    YEAR_2008 = 2008
    YEAR_2009 = 2009
    YEAR_2010 = 2010
    YEAR_2011 = 2011
    YEAR_2012 = 2012
    YEAR_2013 = 2013
    YEAR_2014 = 2014
    YEAR_2015 = 2015
    YEAR_2016 = 2016
    YEAR_2017 = 2017
    YEAR_2018 = 2018
    YEAR_2019 = 2019
    YEAR_2020 = 2020
    YEAR_2021 = 2021
    YEAR_2022 = 2022
    YEAR_2023 = 2023
    YEAR_2024 = 2024


class Quarter(IntEnum):
    Q1 = 1
    Q2 = 2
    Q3 = 3
    Q4 = 4

class Statements(IntEnum):
    CASHFLOW = 1
    BALANCE = 2
    INCOME = 3
    ALL = 4



class BaseDriver():
    def __init__(self, tick):
        self.quarter: int
        self.tick = tick
        self.page: WebElement
        self.data: WebElement
        self.table: WebElement
        self.year_choice: int
        self.statement = None
        self.working_dir = os.environ.get('CURRENT_DIR') 
    def print_attr(self):
        print(f"{self.tick}")
        print(f"{self.quarter}")
    
    def create_driver(self):
        options = webdriver.FirefoxOptions()
        
        options.add_argument("--headless")
        options.add_argument("--no-sandbox")
        options.add_argument("--diable-dev-shm-usage")

        load_dotenv()
        
        WEB_DRIVER_LOCATION = os.getenv('WEB_DRIVER')

        options.binary_location = WEB_DRIVER_LOCATION

        self.driver = webdriver.Firefox(options=options)

        print("driver loaded")

        url = "https://www.sec.gov/edgar/search"

        self.driver.get(url)

        self.driver.implicitly_wait(5)

        search = self.driver.find_element(By.ID, "entity-short-form")

        search.send_keys(self.tick)
        time.sleep(1)

        search_result = search.find_element(By.XPATH, "..//table/tr/td")

        search_result.click()
        time.sleep(2)

        match self.quarter:
            case Quarter.Q1:
                self.navigate_search(self.year_choice)
            case Quarter.Q2:
                self.navigate_search(self.year_choice)
            case Quarter.Q3:
                self.navigate_search(self.year_choice)
            case Quarter.Q4:
                self.navigate_search(self.year_choice)

        self.driver.implicitly_wait(10)

        self.driver.find_element(By.CLASS_NAME, "btn-warning").click()

        self.driver.implicitly_wait(10)

        self.driver.switch_to.window(self.driver.window_handles[1])

        return self.driver
    
    def navigate_search(self, year):
        match self.quarter:
            case Quarter.Q1:
                print("finding Q1 report")
                date_from = self.driver.find_element(By.ID, "date-from")
                date_from.click()
                date_from.send_keys(Keys.CONTROL + "a")
                date_from.send_keys(Keys.DELETE)
                date_from.send_keys(f"{year}-01-01")
                date_to = self.driver.find_element(By.ID, "date-to")
                date_to.click()
                date_to.send_keys(Keys.CONTROL + "a")
                date_to.send_keys(Keys.DELETE)
                date_to.send_keys(f"{year}-02-28")
                self.driver.find_element(By.XPATH, "//html").click()
                self.driver.refresh()
                time.sleep(3)
                self.driver.find_element(
                    By.LINK_TEXT, "10-Q (Quarterly report)").click()
            case Quarter.Q2:
                print("finding Q2 report")
                date_from = self.driver.find_element(By.ID, "date-from")
                date_from.click()
                date_from.send_keys(Keys.CONTROL + "a")
                date_from.send_keys(Keys.DELETE)
                date_from.send_keys(f"{year}-03-01")
                date_to = self.driver.find_element(By.ID, "date-to")
                date_to.click()
                date_to.send_keys(Keys.CONTROL + "a")
                date_to.send_keys(Keys.DELETE)
                date_to.send_keys(f"{year}-05-31")
                self.driver.find_element(By.XPATH, "//html").click()
                self.driver.refresh()
                time.sleep(3)
                self.driver.find_element(
                    By.LINK_TEXT, "10-Q (Quarterly report)").click()
            case Quarter.Q3:
                print("finding Q3 report")
                date_from = self.driver.find_element(By.ID, "date-from")
                date_from.click()
                date_from.send_keys(Keys.CONTROL + "a")
                date_from.send_keys(Keys.DELETE)
                date_from.send_keys(f"{year}-06-01")
                date_to = self.driver.find_element(By.ID, "date-to")
                date_to.click()
                date_to.send_keys(Keys.CONTROL + "a")
                date_to.send_keys(Keys.DELETE)
                date_to.send_keys(f"{year}-08-31")
                self.driver.find_element(By.XPATH, "//html").click()
                self.driver.refresh()
                time.sleep(3)
                self.driver.find_element(
                    By.LINK_TEXT, "10-Q (Quarterly report)").click()
            case Quarter.Q4:
                time.sleep(3)
                print("finding annual report")
                date_from = self.driver.find_element(By.ID, "date-from")
                date_from.click()
                date_from.send_keys(Keys.CONTROL + "a")
                date_from.send_keys(Keys.DELETE)
                date_from.send_keys(f"{year}-09-01")
                date_to = self.driver.find_element(By.ID, "date-to")
                date_to.click()
                date_to.send_keys(Keys.CONTROL + "a")
                date_to.send_keys(Keys.DELETE)
                date_to.send_keys(f"{year}-12-31")
                self.driver.find_element(By.XPATH, "//html").click()
                self.driver.refresh()
                time.sleep(3)
                self.driver.find_element(
                    By.LINK_TEXT, "10-K (Annual report)").click()
                
                
    def find_page_link(self, table):
        possible_elements = ['font', 'h1', 'span']

        err_counter = 0

        try: 
            link = self.follow_link()
            if link is None:
                print("Cannot find link!")
            else:
                top_page = link
                for i in range(len(possible_elements)):
                    try:
                        self.driver.implicitly_wait(0.1)
                        self.page = top_page.find_element(
                            By.XPATH, f"./%s[contains(text(),'{table}')]" % possible_elements[i])
                        print(self.page.get_attribute("innerHTML"))
                        return self.page
                    except NoSuchElementException:
                        err_counter += 1
                        if err_counter == range(len(possible_elements)):
                            raise Exception(
                                "too many tries, unable to find element")
                        else:
                            print("cannot find element page containing table")
                            pass

        except NoSuchElementException as e:
            print(e.msg)
            
            
    def find_page_nolink(self, table: str):
        possible_elements = ['font', 'h1', 'span','b']
        print("finding page without link")
        err_counter = 0
        try:
            for i in range(len(possible_elements)):
                try:
                    self.driver.implicitly_wait(0.1)
                    self.page = self.driver.find_element(
                        By.XPATH, f"//%s[contains(text(),'{table}')]" % possible_elements[i])
                    print(self.page.get_attribute("innerHTML"))
                except NoSuchElementException:
                    err_counter += 1
                    if err_counter == range(len(possible_elements)):
                        raise Exception(
                            "too many tries, unable to find element")
                    else:
                        print("cannot find element page containing table")
                        pass

        except NoSuchElementException as e:
            print(e.msg)
    