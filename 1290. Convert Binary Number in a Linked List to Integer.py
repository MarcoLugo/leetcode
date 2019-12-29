# Given head which is a reference node to a singly-linked list. The value of
# each node in the linked list is either 0 or 1. The linked list holds the 
# binary representation of a number.

# Return the decimal value of the number in the linked list.

# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def getDecimalValue(self, head: ListNode) -> int:
        buffer = []
        result = 0
        
        while head.next:
            buffer.append(head.val)
            head = head.next
            
        buffer.append(head.val)
        
        n_bits = len(buffer)
        
        for i, v in enumerate(buffer):
            power = n_bits - i - 1
            result += v * (2 ** power)
            
        return result