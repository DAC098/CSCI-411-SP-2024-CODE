all: assignment_01 assignment_02

assignment_01:
	$(MAKE) -C ./assignment_01

assignment_02:
	$(MAKE) -C ./assignment_02

assignment_03:
	$(MAKE) -C ./assignment_03

clean:
	$(MAKE) -C ./assignment_01 clean
	$(MAKE) -C ./assignment_02 clean
	$(MAKE) -C ./assignment_03 clean

.PHONY: all assignment_01 assignment_02 assignment_03 clean
