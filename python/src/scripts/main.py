from classes.aapl import AaplDriver
from classes.aapl import validate_input


def scrape(year, quarter):

    instance = AaplDriver(tick = "AAPL")

    # is_valid = False

    # while (not is_valid):
    #     print("Enter the year to seach between 2001-present:")
    #     instance.year_choice = input()
    #
    #     print("Enter in the following Quarter")
    #     print("1: Q1")
    #     print("2: Q2")
    #     print("3: Q3")
    #     print("4: Q4")
    #
    #     str_choice = input()
    #
    #     if validate_input(str_choice):
    #         instance.quarter = int(str_choice)
    #         is_valid = True
    #     else:
    #         is_valid = False
    instance.year_choice = year 
    instance.quarter = quarter  
    
    instance.print_attr()
    instance.create_driver()
    tables = ['FLOWS','BALANCE','INCOME']

    for table in tables:
        result = instance.find_page_link(table)
    
        if result is None:
            instance.find_page_nolink(table)
            instance.find_table() 
            instance.process_table(table)
        else:
            instance.find_table()
            instance.process_table(table)
    instance.quit()

if __name__ == '__main__':
    scrape(2020,'Q4')
