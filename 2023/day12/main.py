from collections import deque
import re

def find_next_symbol(record, index):
    hashI = record.find("#", index)
    qI = record.find("?", index)

    if hashI == -1 and qI == -1:
        return -1
    elif hashI == -1:
        return qI
    elif qI == -1:
        return hashI
    else:
        return min(hashI, qI)

def dfs(record, rules, result, index):

    # It should check if all rules are fulfilled
    if not rules:
        # All possible ? are replaced, add to result
        result.append(record)
        return result

    # Search for next valid symbol from the index we finished before
    sI = find_next_symbol(record, index) 

    if sI != -1:
        qStart = sI

        rule = rules.popleft()
        while record[sI] == "?":
            qDiff = sI - qStart
            if "." not in record[sI: sI + rule] :
                n_record = record[:qStart] + "."*qDiff + "#"*rule + "." + record[sI + rule + 1:]
                
                    dfs(n_record, deque(rules), result, sI + rule)
            
            sI += 1

    return result    

if __name__ == "__main__":
    # Mark rules as fulfilled or not
    result = dfs("?#?#?#?#?#?#?#?", rules=deque([1,3,1,6]), result=[], index=0)
    print(len(result))