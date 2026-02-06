class Solution:
    def subdomainVisits(self, cpdomains: List[str]) -> List[str]:
        store = defaultdict(int)

        for doms in cpdomains:
            num, domain = doms.split()
            visits = int(num)
            store[domain] += visits

            dot = domain.find(".")
            while dot != -1:
                sub = domain[dot+1:]
                store[sub] += visits
                dot = domain.find(".", dot+1)
        
        return[f"{count} {domain}" for domain, count in store.items()]
            