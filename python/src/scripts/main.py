from classes.aapl import AaplDriver
from classes.nvda import NvdaDriver


def scrape(year, quarter):
    instance = AaplDriver("AAPL")

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
