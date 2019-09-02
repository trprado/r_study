suppressMessages(library(dplyr, quietly = TRUE))
suppressMessages(library(tidyr, quietly = TRUE))
suppressMessages(library(ggplot2, quietly = TRUE))
suppressMessages(library(gridExtra, ))

data("diamonds")
str(diamonds)

# 1:
ggplot(data = diamonds,
       aes(x = log(price))) +
    geom_histogram(aes(color = cut, fill = cut), binwidth = 0.15 ) +
    scale_x_continuous(limits = c(5.5,10), breaks = c(7, 9), labels = c(1000, 10000)) +
    scale_y_continuous(limits = c(0, 700)) +
    facet_wrap(~color, ncol = 3)

# 2:
ggplot(data = diamonds,
       aes(x = table, y = price)) +
    geom_jitter(aes(color = cut)) +
    scale_x_continuous(limits = c(50, 80), breaks = seq(50, 80, 2))

# 3:
# **Ideal?** 53 - 57
# **Premium?** 58 - 52

# 4:
diamonds$volume <- diamonds$x * diamonds$y * diamonds$z

# diamonds$clarity <- factor(diamonds$clarity, levels = rev(levels(diamonds$clarity)))

ggplot(data = diamonds,
       aes(x = volume, y = price)) +
    geom_point(aes(color = clarity)) +
    scale_y_log10(breaks = c(1000, 10000)) +
    scale_x_continuous(limits = c(quantile(diamonds$volume, 0.01),
                                  quantile(diamonds$volume, .99))) +
    scale_color_brewer(type = 'div')


# 5:
pf <- read.csv('pseudo_facebook.tsv', sep = '\t')
pf <- tbl_df(pf)

pf$prop_initiated <- ifelse(pf$friend_count == 0, 0, pf$friendships_initiated / pf$friend_count)
head(pf)
tail(pf)


# 6:
pf$year_joined <- floor(2014 - pf$tenure/365)
pf$year_joined_bucket <- cut(pf$year_joined, breaks = c(2004, 2009, 2011, 2012, 2014), )

ggplot(data = subset(pf, !is.na(year_joined_bucket) & tenure != 0),
       aes(x = tenure, y = prop_initiated)) +
    geom_line(aes(color = year_joined_bucket), stat = 'summary' , fun.y = median)

# 7:
ggplot(data = subset(pf, !is.na(year_joined_bucket) & tenure != 0),
       aes(x = tenure, y = prop_initiated)) +
    geom_line(aes(color = year_joined_bucket), stat = 'summary' , fun.y = median) +
    geom_smooth()

# 8:
# 2012-2014

# 9:
by(pf$prop_initiated, pf$year_joined_bucket, mean)
# 0.6430

# 10:
ggplot(data = diamonds,
       aes(x = cut, y = price/carat)) +
    geom_point(aes(color = color)) +
    facet_wrap(~clarity) +
    scale_color_brewer(type = 'div')

# Gapminder
df_elect <- read.csv('electricity_consumption_total_clean.csv')
df_elect <- tbl_df(df_elect)

table(df_elect$year)
df_elect$decade <- cut(df_elect$year, breaks = c(1959, 1969, 1979, 1989, 1999, 2009, 2011), labels = c(1960, 1970, 1980, 1990, 2000, 2010))

# 1:
south_countries <- c('Argentina', 'Bolivia', 'Brazil', 'Chile', 'Colombia', 'Ecuador', 'Guyana', 'Paraguay', 'Peru', 'Suriname', 'Uruguay', 'Venezuela')

ggplot(data = subset(df_elect,
                     !is.na(kWh)
                     & continent == 'Americas'
                     &country %in% south_countries),
       aes(x = year, y = kWh)) +
    geom_line(aes(color = country), stat = 'summary', fun.y = median) +
    scale_y_log10() +
    geom_smooth()

# 2:
ggplot(data = subset(df_elect, !is.na(kWh) & continent != ''),
       aes(x = year, y = kWh)) +
    geom_line(aes(color = country), stat = 'summary', fun.y = median) +
    scale_y_continuous(limits = c(quantile(df_elect$kWh, 0.05, na.rm = TRUE),
                                  quantile(df_elect$kWh, 0.95, na.rm = TRUE))) +
    geom_smooth()
