DEBUG ?= 0

RM = rm -rf
MKDIR = mkdir -p
CXX = g++
ifeq ($(DEBUG), 0)
CXX_FLAGS = -std=c++14\
			-Os\
			-Wall\
			-Wextra\
			-Wshadow
else
CXX_FLAGS = -std=c++14\
			-O0\
			-Wall\
			-Wextra\
			-Wshadow\
			-g\
			-D_GLIBCXX_DEBUG\
			-fsanitize=address\
			-fsanitize=undefined
endif
INC_FLAGS = -I$(inc_dir) -I$(header_dir)

src_dir = src
lib_dir = lib
inc_dir = include
out_dir = out
obj_dir = $(out_dir)/obj
header_dir = $(src_dir)/headers
impl_dir = $(src_dir)/impl

src = $(wildcard $(src_dir)/*.cpp)
lib_src = $(wildcard $(lib_dir)/*.cpp)
impl_src = $(wildcard $(impl_dir)/*.cpp)

src_obj = $(src:$(src_dir)/%.cpp=$(obj_dir)/%.o)
lib_obj = $(lib_src:$(lib_dir)/%.cpp=$(obj_dir)/%.o)
impl_obj = $(impl_src:$(impl_dir)/%.cpp=$(obj_dir)/%.o)

target = $(out_dir)/v.out

.PHONY: default all help install run clean

default: install
all: $(obj_dir) run

$(obj_dir):
	$(MKDIR) $(obj_dir)

$(obj_dir)/%.o: $(src_dir)/%.cpp
	$(CXX) $(INC_FLAGS) $(CXX_FLAGS) -c -o $@ $<

$(obj_dir)/%.o: $(lib_dir)/%.cpp
	$(CXX) $(INC_FLAGS) $(CXX_FLAGS) -c -o $@ $<

$(obj_dir)/%.o: $(impl_dir)/%.cpp
	$(CXX) $(INC_FLAGS) $(CXX_FLAGS) -c -o $@ $<

$(target): $(src_obj) $(impl_obj) $(lib_obj)
	$(CXX) $(INC_FLAGS) $(CXX_FLAGS) -o $@ $^

help:
	$(info `make help`: keluarkan pesan bantuan ini)
	$(info `make`: jalankan unit testing compile program)
	$(info `make all`: jalankan unit testing lalu jalankan program)
	$(info `make install`: jalankan unit testing lalu compile program)
	$(info `make run`: compile & jalankan program)

install: $(obj_dir) $(target)

run: $(target)
	./$<

clean:
	$(RM) $(target) $(test_target) $(obj_dir)/*

