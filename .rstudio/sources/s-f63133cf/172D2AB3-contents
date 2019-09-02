library('ggplot2')
data('diamonds')

summary(diamonds)
?diamonds
dim(diamonds)
str(diamonds)
levels(diamonds$color)

ggplot(aes(x = price), data = subset(diamonds, !is.na(price))) +
    geom_histogram(aes(color = color)) +
    # scale_x_continuous(limits = c(0, 20000), breaks = seq(0, 20000, 1000)) +
    scale_x_log10(limits = c(326, 20000))

dim(subset(diamonds, price < 250))
dim(subset(diamonds, price < 500))
dim(subset(diamonds, price >= 15000))

ggplot(aes(x = price), data = subset(diamonds, !is.na(price))) +
    geom_histogram(aes(color = color), binswidth = 1) +
    scale_x_continuous(limits = c(500, 1000), breaks = seq(500, 1000, 10)) +
    theme(axis.text.x = element_text(angle = 90)) +
    ggsave('diamonds_price_hist.png')

ggplot(aes(x = price), data = subset(diamonds, !is.na(price))) +
    geom_histogram(aes(color = color), binswidth = 50) +
    scale_x_continuous(limits = c(0, 20000), breaks = seq(0, 20000, 1000)) +
    facet_grid(rows = vars(cut)) +
    theme(axis.text.x = element_text(angle = 45)) +
    ggsave('diamonds_price_hist_by_cut.png')

by(diamonds$price, diamonds$cut, summary)

ggplot(aes(x = price), data = subset(diamonds, !is.na(price))) +
    geom_histogram() +
    facet_wrap(~cut, scales = 'free_y')

ggplot(aes(x = price/carat), data = subset(diamonds, !is.na(price))) +
    geom_histogram(binwidth = 0.1) +
    scale_x_log10() +
    facet_wrap(~cut, scales = 'free_y') +
    theme(axis.text.x = element_text(angle = 90)) +
    ggsave('diamonds_price_per_carat_by_cut.png')

library(gridExtra)
p1 <- ggplot(aes(y = price, x = cut), data = diamonds) +
    geom_boxplot()

p2 <- ggplot(aes(y = price, x = clarity), data = diamonds) +
    geom_boxplot()

p3 <- ggplot(aes(y = price, x = color), data = diamonds) +
    geom_boxplot()
grid.arrange(p1, p2, p3)
g <- arrangeGrob(p1, p2, p3)
ggsave('diamon_boxplots_bu_cut_clarity_and_color.png', g)

by(diamonds$price, diamonds$cut, summary)
by(diamonds$price, diamonds$clarity, summary)
by(diamonds$price, diamonds$color, summary)

IQR(subset(diamonds, color == 'D')$price)
IQR(subset(diamonds, color == 'J')$price)

ggplot(aes(y = carat, x = color), data = diamonds) +
    geom_boxplot() +
    ggsave('diamonds_boxplot_carat_per_color.png')

by(diamonds$carat, diamonds$color, summary)


ggplot(aes(x = carat), data = diamonds) +
    geom_freqpoly(binwidth = 0.1) +
    scale_x_continuous(limits = c(0, 2), breaks = seq(0, 2, 0.1)) +
    scale_y_continuous(breaks = seq(0, 15000, 1000))
table(diamonds$carat)



# install.packages(c('dplyr', 'tidyr')) # Libs to data wrangling
library('dplyr')
library('tidyr')

# install.packages("openxlsx", dependencies = TRUE) # Lib to load xlsx
library(openxlsx)

#  Open xlsx
df_electric <- read.xlsx(xlsxFile = 'Indicator_Electricity consumption total.xlsx', sheet = 1)
# head(df_eletric)

# Convert columns to one column of years
df2_electric <- gather(data = df_electric, key = 'year', value = 'kWh', 2:ncol(df_electric))

# NA in kWh to 0
# df2_electric <- replace_na(df2_electric, list(kWh = 0))

# Change to tbl
df_elec <- tbl_df(df2_electric)

# Rename first column
df_elec <- rename(.data = df_elec, 'country' = 'Electricity.consumption,.total.(kWh)')

# install.packages('countrycode')

# Add column continent name
library(countrycode)
df_elec$continent <- countrycode(sourcevar = df_elec$country,
                                origin = 'country.name',
                                destination = 'continent')

head(df_elec)
tail(df_elec)

# change year to numeric
df_elec$year <- as.numeric(df_elec$year)

write.csv(df_elec, file = 'electricity_consumption_total_clean.csv', na = '', row.names = FALSE)

library(ggplot2)

summary(df_elec)

# Plot 1: Consume in kWh per continent of 1960 to 2010
ggplot(aes(x = year, y = kWh), data = subset(df_elec, !is.na(kWh) & !is.na(continent))) +
    geom_bar(stat = "identity") +
    scale_x_continuous(limits = c(1960, 2010), breaks = seq(1960, 2010, 5)) +
    facet_wrap(~continent, ncol=2, scales = 'free') +
    ggtitle('Consume in kWh per cotinent of 1960 to 2010')+
    ggsave('consume_in_kWh_per_continent_of_1960-2010.png')

# Plot 2:
ggplot(aes(x = kWh), data = subset(df_elec, !is.na(kWh) & !is.na(continent))) +
    geom_freqpoly(binwidth = 0.1) +
    scale_x_log10() +
    facet_wrap(~year, ncol = 5, scales = 'free') +
    ggsave('freq_consume_kWh_per_year.png', scale = 2)

df_elec$kWh=as.numeric(levels(df_elec$kWh))[df_elec$kWh]
# Plot 3:
ggplot(aes(x = continent, y = kWh), data = subset(df_elec, !is.na(kWh) & !is.na(continent))) +
    geom_boxplot() +
    scale_y_log10() +
    ggsave('boxplot_consume_per_continent.png')

################################################################################
library('dplyr')
library('tidyr')

df_birthdays <- read.csv('birthdays_example.csv')
str(df_birthdays)
summary(df_birthdays)
class(df_birthdays$dates)
df_birthdays <- tbl_df(df_birthdays)
df_birthdays$dates <- as.Date(df_birthdays$dates, format = "%m/%d/%Y")
birthdays_group <- group_by(.data = df_birthdays, dates)
df_birthdays <- summarise(birthdays_group, count = length(dates))
# df_birthdays <- rename(.data = df_birthdays, 'count' = 'length(dates)')

# 1
df_birthdays %>% filter(dates == '84-10-11')

# 2
df_month <- df_birthdays %>% mutate(month = format(dates, '%m')) %>% group_by(month) %>% summarise(count = length(month))
df_month

# 3
df_day <- df_birthdays %>% mutate(day = format(dates, '%d')) %>% group_by(day) %>% summarise(count = length(day))
df_day

# 4
df_unique_birth <- df_birthdays %>% distinct(dates) %>% summarise(count = length(dates))
df_unique_birth
