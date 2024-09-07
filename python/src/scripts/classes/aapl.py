from enum import IntEnum
from enum import StrEnum
from typing import Optional
from selenium import webdriver
from selenium.webdriver import FirefoxOptions
from selenium.webdriver.remote.webdriver import WebDriver
from selenium.webdriver.remote.webelement import WebElement
from selenium.webdriver.common.by import By
from selenium.common.exceptions import NoSuchElementException
from selenium.webdriver.common.keys import Keys
import os
import csv
import time
import re
import logging


def enquiry(list):
    if len(list) == 0:
        return 0
    else:
        return 1
def find_pattern(text: str, pattern: str) -> Optional[str]:
    match = re.search(fr"pattern", text)
    if match:
        return match.group()
    return None

def validate_input(str_input):
    is_valid = False
    try:
        int_input = int(str_input)
    except Exception as e:
        print("Unable to convert type")
        return
    if int_input > len(Quarter):
        print(f"input is higher than available range: {len(Quarter)}")
    else:
        is_valid = True
    return is_valid


def remove_patterns(data):
    patterns = [
        r'\n',
        r'<br>',
        r'&nbsp;',
    ]

    for pattern in patterns:
        mod_data = re.sub(pattern, "", data)
        return mod_data


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


class AaplDriver():
    def __init__(self, tick):
        self.driver: WebDriver
        self.tick: str = tick
        self.quarter: int
        self.page: WebElement
        self.data: WebElement
        self.table: WebElement
        self.year_choice: int
        self.statement = None

    def print_attr(self):
        print(f"{self.tick}")
        print(f"{self.quarter}")
        # print(f"{self.page}")
        # print(f"{self.data}")
        # print(f"{self.table}")

    def follow_link(self):
        found = False
        possible_elements = ["a","span","font"]
        err_counter = 0
        pattern = r"#(.*?).*$" 
        for i in range(len(possible_elements)):
            self.driver.implicitly_wait(1)
            print(possible_elements[i])
            if possible_elements[i] == "span":
                element = self.driver.find_element(By.XPATH, "//a[%s[text()='Financial Statements']]" % possible_elements[i]).get_attribute("href") 
            else:
                element = self.driver.find_element(By.XPATH, "//%s[contains(text(), 'Financial Statements')]" % possible_elements[i]).get_attribute("href")


            if element is None:
                print("href not found")
            else:
                href = element
                match = re.search(pattern, href)
                if match:
                    extr_string = match.group()[1:]
                    try:
                        self.driver.implicitly_wait(2)
                        found_page = self.driver.find_element(By.ID, extr_string)
                        found = True
                    except Exception:
                        print("element not found")
                        pass
                    if not found:
                        print("finding element by name")
                        found_page = self.driver.find_element(By.NAME, extr_string)
                        if found_page is None:
                            print("page not found")
                        else:
                            print("found link")
                            return found_page
             
        
        

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

    def create_driver(self):
         

        options = webdriver.FirefoxOptions()
        
        # options.add_argument("--headless")

        options.binary_location = "/usr/bin/firefox"

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
    
    

    def find_table(self):
        possible_elements = ['div','p']
        for element in possible_elements:

            try:
                self.driver.implicitly_wait(3)
                self.table = self.page.find_element(
                    By.XPATH, f"../following::{element}/table")
                print("found table")
                return 
                # print(table.get_attribute("innerHTML"))
            except Exception as e:
                print(str(e))
                # raise Exception("cannot find table!")
            try:
                self.driver.implicitly_wait(3)
                self.table = self.page.find_element(
                    By.XPATH, f"../following-sibling::{element}/{element}/table")
                return 
            except Exception as e:
                print(str(e))
            try:
                self.driver.implicitly_wait(3)
                self.table = self.page.find_element(
                    By.XPATH, "../../following-sibling::table")
                return 
            except Exception as e:
                print(str(e))

    def process_table(self, table: str):
        directory_path = f'output/AAPL/{self.year_choice}/Q{self.quarter}/'
        os.makedirs(directory_path, exist_ok=True);
        rows = self.table.find_elements(By.TAG_NAME, "tr")
        with open(f"output/AAPL/{self.year_choice}/Q{self.quarter}/{table}.csv", "w", newline='') as csvfile:
            wr = csv.writer(csvfile)
            table_data = []
            for x in range(len(rows)):
                row = rows[x].find_elements(By.XPATH, ".//td")
                # print(row[0].get_attribute("innerHTML"))
                row_data = []
                for i in range(len(row)):
                    try:
                        self.driver.implicitly_wait(0.01)
                        row_info = row[i].find_elements(By.TAG_NAME, "span")
                        for cell in range(len(row_info)):
                            info = row_info[cell].text
                    # Remove unwanted patterns
                            info = re.sub(r'\n', '', info)
                            info = re.sub(r'<br>', '', info)
                            info = re.sub(r'&nbsp;', '', info)
                            info = re.sub(r'^\(', '-', info)
                            info = re.sub(r'\)', '', info)
                            info = re.sub(r'\$', '', info)
                            # print(info)
                            # time.sleep(2)
                            row_data.append(info)
                            if not bool(info.strip()):
                                row_data.pop()
                            # if info == row_data[-2]:
                            #     print("popping data")
                            #     row_data.pop()
                    except NoSuchElementException as e:
                        print(e.msg)
                        pass
                # print(row_data)
                    # remove duplicate data
                    try:
                        if len(row_data) > 1:
                            # print(row_data)
                            for i in range(len(row_data)):
                                # print(row_data[i])
                                # print(len(row_data))
                                current_element = row_data[i]
                                previouse_element = row_data[i - 1]
                                if current_element == previouse_element:
                                    row_data.pop(i)
                    except:
                        pass
                if enquiry(row_data):
                    # print(row_data)
                    table_data.append(row_data)
            # print(len(table_data))
            for i in range(len(table_data)):
                print(table_data[i])
                wr.writerow(table_data[i])

            if not enquiry(table_data):
                for x in range(len(rows)):
                    row = rows[x].find_elements(By.XPATH, ".//td")
                    # print(row[0].get_attribute("innerHTML"))
                    row_data = []
                    for i in range(len(row)):
                        try:
                            # print("we are looking for font")
                            self.driver.implicitly_wait(0.01)
                            row_info = row[i].find_elements(
                                By.TAG_NAME, "font")
                            for cell in row_info:

                                info = cell.text
                            # Remove unwanted patterns
                                info = re.sub(r'\n', '', info)
                                info = re.sub(r'<br>', '', info)
                                info = re.sub(r'&nbsp;', '', info)
                                info = re.sub(r'^\(', '-', info)
                                info = re.sub(r'\)', '', info)
                                info = re.sub(r'\$', '', info)
                                row_data.append(info)
                                if not bool(info.strip()):
                                    row_data.pop()
                        except NoSuchElementException as e:
                            print(e.msg)
                            pass
                    if enquiry(row_data):
                        # print(row_data)
                        table_data.append(row_data)
                # print(len(table_data))
                for i in range(len(table_data)):
                    print(table_data[i])
                    wr.writerow(table_data[i])

    def quit(self):
        self.driver.quit()
