# ========================================
# Makefile genérico para C++ com detecção
# automática de OpenMP
#
# Uso:
#   make FILE=main.cpp
#   make FILE=teste.cc
# ========================================

CXX := g++

CXXFLAGS := -Wall -Wextra -O2

ifndef FILE
$(error Use assim: make FILE=main.cpp)
endif

TARGET := $(basename $(FILE))

USE_OPENMP := $(shell grep -E -i 'omp_|<omp\.h>|#pragma[[:space:]]+omp' $(FILE) >/dev/null && echo 1 || echo 0)

ifeq ($(USE_OPENMP),1)
CXXFLAGS += -fopenmp
$(info [INFO] OpenMP detectado. Adicionando -fopenmp)
endif

all: $(TARGET)


$(TARGET): $(FILE)
	$(CXX) $(CXXFLAGS) -o $(TARGET) $(FILE)
