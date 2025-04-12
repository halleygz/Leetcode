/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
class Solution {
public:
    ListNode* mergeTwoLists(ListNode* list1, ListNode* list2) {
        ListNode* dummy = new ListNode(0);
        ListNode* res = dummy;
        while (list1 != nullptr && list2 != nullptr) {
            if (list1->val > list2->val) {
                res->next = list2;
                list2 = list2->next;
            } else {
                res->next = list1;
                list1 = list1->next;
            }
            res = res->next;
        }
        if (list1 != nullptr) {
            res->next = list1;
        }

        if (list2 != nullptr) {
            res->next = list2;
        }
        ListNode* head = dummy->next;
        delete dummy;
        return head;
    }
};