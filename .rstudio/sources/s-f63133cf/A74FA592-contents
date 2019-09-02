# 0:
library(dplyr)
library(tidyr)
library(ggplot2)
library(gridExtra)

data(diamonds)
?diamonds
df <- tbl_df(diamonds)
df

# 1:
ggplot(data = df, aes(x = x, y = log10(price))) +
    geom_jitter(alpha = 0.05) +
    scale_x_continuous(limits = c(3, 9), breaks = seq(3, 9, 1))

# 2: Price e X são altamente correlacionados ou seja, quanto maior o tamanho dos diamantes maior sera seu valor.

# 3:
round(cor(x = df$x, y = df$price), 2)
round(cor(x = df$y, y = df$price), 2)
round(cor(x = df$z, y = df$price), 2)

# 4:
ggplot(data = df, aes(x = price, y = depth)) +
    geom_jitter(alpha = 0.05) +
    scale_x_continuous(limits = c(326, quantile(df$price, 0.95)),
                       breaks = seq(326, quantile(df$price, 0.95), 250)) +
    theme(axis.text.x = element_text(angle = 45))

# 5
ggplot(data = df, aes(x = depth, y = price)) +
    geom_point(alpha = 1/100) +
    scale_x_continuous(breaks = seq(55, 70, 2))

# 6: 59, 64

# 7:
cor.test(df$price, df$depth)
# Tem uma baixa correlação, o que pode ser não significativo

# 8:
ggplot(data = df, aes(x = carat, y = price)) +
    geom_jitter() +
    xlim(0, quantile(df$carat, .99)) +
    ylim(0, quantile(df$price, .99))

cor.test(df$price, df$carat)

# 9:
df$volume <- df$x * df$y * df$z

ggplot(data = df, aes(x = volume, y = price)) +
    geom_jitter()
    # xlim(quantile(df$volume, .01), quantile(df$volume, .99))

# 10: A medida que se aumenta o volume se aumenta o preço. Existem outliers onde o volume é zero para grandes valores.

# 11 :
with(subset(df, volume > 0 & volume <= 800), round(cor(price, volume), 2))

# 12:
ggplot(data = subset(df, volume > 0 & volume <= 800),
       aes(x = volume, y = price)) +
    geom_jitter(alpha = 0.05) +
    geom_smooth(method = 'lm')

m0 <- lm(with(subset(df, volume > 0 & volume <= 800), df$price~df$volume))
summary(m0)
par(mfrow = c(2,2))
plot(m0)

ks.test(resid(m0), 'pnorm')

# Existe um alta correlação o que torna o modelo bom para se prever o preço dos diamantes.

# 12:

df_diamonds_by_clarity <- df %>%
    group_by(clarity) %>%
    summarise(mean_price = mean(price),
              median_price = median(price),
              min_price = min(price),
              max_price = max(price),
              n = n()) %>%
    arrange(clarity)

head(df_diamonds_by_clarity)

# 13:

diamonds_by_clarity <- group_by(df, clarity)
diamonds_mp_by_clarity <- summarise(diamonds_by_clarity, mean_price = mean(price))

diamonds_by_color <- group_by(df, color)
diamonds_mp_by_color <- summarise(diamonds_by_color, mean_price = mean(price))


p1 <- ggplot(data = diamonds_mp_by_clarity, aes(x = clarity, y = mean_price)) +
    geom_col()

p2 <- ggplot(data = diamonds_mp_by_color, aes(x = color, y = mean_price)) +
    geom_col()

grid.arrange(p1, p2)

# 14: Ambos possuem uma distribuição exponencial. Em relação a color temos um
# crescimento suave no preço, enquanto a claridade existe um pico em SI2 é uma
# queda do preço conforme a claridade aumenta.

# 15:
df_elec <- read.csv('electricity_consumption_total_clean.csv', na.strings = "")
df_elec <- tbl_df(df_elec)

df_elec <- subset(df_elec, !is.na(kWh) & !is.na(continent))

# P1:
ggplot(data = df_elec, aes(x = year, y = kWh)) +
    geom_jitter(alpha = 0.05) +
    geom_smooth(method = 'lm') +
    ylim(0, quantile(df_elec$kWh, 0.99)) +
    scale_x_continuous(breaks = seq(1960, 2011, 1)) +
    scale_y_log10() +
    theme(axis.text.x = element_text(angle = 45, hjust = 1)) +
    ggsave('plot1_year_by_kWh.png')

# P2:
ggplot(data = df_elec, aes(x = year, y = kWh)) +
    geom_jitter() +
    geom_smooth(method = 'lm') +
    ylim(0, quantile(df_elec$kWh, 0.99)) +
    scale_x_continuous(breaks = seq(1960, 2011, 5)) +
    facet_wrap(~continent, ncol = 3) +
    theme(axis.text.x = element_text(angle = 45, hjust = 1)) +
    ggsave('plot2_year_by_kWh_per_continent.png')

# P3:

# top5<-list()
# continentes<-unique(df_elec$continent)
# for(i in 1:length(continentes)){
#     set1<-subset(df_elec, df_elec$continent==levels(continentes)[i])
#     set2<-filter(set1, set1$kWh >= quantile(set1$kWh, 0.95))
#
#     top5[[i]]<-ggplot(data = set2,
#            aes(x = year, y = kWh)) +
#         geom_jitter() +
#         geom_smooth(method = 'lm') +
#         ylim(0, quantile(set2$kWh, 0.99)) +
#         scale_x_continuous(breaks = seq(1960, 2011, 1)) +
#         facet_wrap(continent~country, ncol = 4, scales = 'free') +
#         theme(axis.text.x = element_text(angle = 90, hjust = 1))
#
# }
# grid.arrange(top5[[1]], top5[[2]], top5[[3]], top5[[4]], top5[[5]], ncol=2)

elect_most_comsume <- df_elec %>%
    group_by(continent) %>%
    filter(kWh >= quantile(kWh, .95))
country_most_consume <- as.vector(unique(elect_most_comsume$country))

df_most_consyme_by_country <- do.call("rbind",
        lapply(
            country_most_consume,
            function(x){subset(df_elec, country == x)}))
ggplot(data = df_most_consyme_by_country,
       aes(x = year, y = kWh)) +
    geom_jitter() +
    geom_smooth(method = 'lm') +
    scale_x_continuous(breaks = seq(1960, 2011, 1)) +
    facet_wrap(continent~country, ncol = 3, scales = 'free') +
    theme(axis.text.x = element_text(angle = 90, hjust = 1)) +
    ggsave('plo3_year_by_kWh_per_continent_and_most_95_percent_of_consume.png')

# P4:
df_continent_consume_mean <- df_elec %>%
    group_by(continent, year) %>%
    summarise(mean_kWh_continent_by_year = mean(kWh),
              median_kWh_continent_by_year = median(kWh),
              n = n())

ggplot(data = df_continent_consume_mean,
       aes(x = year, y = mean_kWh_continent_by_year)) +
    geom_jitter() +
    facet_wrap(~continent, ncol = 2, scales = 'free') +
    ggsave('plo4_year_by_mean_kWh_continent_and_year.png')
