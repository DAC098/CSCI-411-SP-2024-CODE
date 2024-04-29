all: assignment_01 assignment_02 assignment_03 assignment_04 assignment_05 tree_vc floyd-warshall leaf_partition huffman_encoding

assignment_01:
	$(MAKE) -C ./assignment_01

assignment_02:
	$(MAKE) -C ./assignment_02

assignment_03:
	$(MAKE) -C ./assignment_03

assignment_04:
	$(MAKE) -C ./assignment_04

assignment_05:
	$(MAKE) -C ./assignment_05

tree_vc:
	$(MAKE) -C ./tree_vc

floyd-warshall:
	$(MAKE) -C ./floyd-warshall

leaf_partition:
	$(MAKE) -C ./leaf_partition

huffman_encoding:
	$(MAKE) -C ./huffman_encoding

clean:
	$(MAKE) -C ./assignment_01 clean
	$(MAKE) -C ./assignment_02 clean
	$(MAKE) -C ./assignment_03 clean
	$(MAKE) -C ./assignment_04 clean
	$(MAKE) -C ./assignment_05 clean
	$(MAKE) -C ./tree_vc clean
	$(MAKE) -C ./floyd-warshall clean
	$(MAKE) -C ./leaf_partition clean
	$(MAKE) -C ./huffman_encoding clean

.PHONY: all assignment_01 assignment_02 assignment_03 assignment_04 assignment_05 tree_vc floyd-warshall leaf_partition huffman_encoding clean
