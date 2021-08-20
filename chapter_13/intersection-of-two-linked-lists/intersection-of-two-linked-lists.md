```java
/**
 * 两个指针都恰好移动了双方的长度
 * 不相交的时候，同时为null
 * 相交的时候，同时为相交点
 */
public class Solution {
    public ListNode getIntersectionNode(ListNode headA, ListNode headB) {
        ListNode l1 = headA;
        ListNode l2 = headB;
        while(l1 != l2) {
            l1 = l1 == null ? headB : l1.next;
            l2 = l2 == null ? headA : l2.next;
        }
        return l1;
    }
}
```