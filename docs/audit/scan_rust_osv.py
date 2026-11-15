import json, tomllib, urllib.request, pathlib
packages=[p for p in tomllib.loads(pathlib.Path("Cargo.lock").read_text())["package"] if p.get("source", "").startswith("registry+")]
body={"queries":[{"package":{"name":p["name"],"ecosystem":"crates.io"},"version":p["version"]} for p in packages]}
req=urllib.request.Request("https://api.osv.dev/v1/querybatch", data=json.dumps(body).encode(), headers={"Content-Type":"application/json"})
with urllib.request.urlopen(req, timeout=60) as r: results=json.load(r)
output=[]
for p, result in zip(packages, results["results"]):
    vulns=[]
    for v in result.get("vulns",[]):
        with urllib.request.urlopen("https://api.osv.dev/v1/vulns/"+v["id"],timeout=60) as r: vulns.append(json.load(r))
    output.append({"name":p["name"],"version":p["version"],"vulns":vulns})
pathlib.Path("/tmp/pyprevent-rust-audit.json").write_text(json.dumps(output,indent=2))
print("Scanned",len(output),"registry packages")
for p in output:
    for v in p["vulns"]: print(p["name"],p["version"],v["id"],v.get("summary"),v.get("affected"))
