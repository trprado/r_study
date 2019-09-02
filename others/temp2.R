library(knitr)
library(dplyr)
library(tidyr)
library(janitor)

library(ggplot2)
library(gridExtra)

library(GGally)
library(scales)
library(memisc)


dados<-read.csv("Computers.csv")[,c(-1)]
head(dados)
dim(dados)

summary(dados)

summary(lm(price~speed+hd+ram+screen+cd+multi+premium+ads+trend, data=dados))
par(mfrow=c(2,2))
plot(lm(price~speed+hd+ram+screen+cd+multi+premium+ads+trend, data=dados))
dim(dados)
