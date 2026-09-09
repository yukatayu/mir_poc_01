"""Check document cross-references and browser usability, not mathematical truth."""
from pathlib import Path
import re,json,ast,hashlib
from bs4 import BeautifulSoup
ROOT=Path(__file__).resolve().parents[1]
master=json.loads((ROOT/'provenance/requirements_v1.json').read_text())
trace=json.loads((ROOT/'REQUIREMENT_TRACE.json').read_text())['requirements']
theorems=json.loads((ROOT/'theory/THEOREM_LEDGER.json').read_text())['theorems']
decisions=json.loads((ROOT/'DECISION_DISPOSITIONS.json').read_text())['decisions']
assert len(trace)==119 and len(set(r['id'] for r in trace))==119
assert {r['id'] for r in trace}=={r['id'] for r in master['requirements']}
assert len(decisions)==30 and {r['id'] for r in decisions}=={r['id'] for r in master['decisions']}
assert len(theorems)==24 and len(set(r['id'] for r in theorems))==24
thids={r['id'] for r in theorems}
for r in trace:
    assert not r['requirement_completed'] and not r['owner_approved']
    assert r['remaining'] and r['F0_3_result'] and set(r['theorems'])<=thids
    assert set(r['next_work_units'])<=set('W'+str(i) for i in range(1,9))
assert all(not r['machine_checked_general_theorem'] for r in theorems)
for r in theorems:
    for f in r['related_files']:
        assert (ROOT/f).exists(),f
html=(ROOT/'REPORT.html').read_text();soup=BeautifulSoup(html,'html.parser')
ids=[n['id'] for n in soup.select('[id]')];assert len(ids)==len(set(ids))
for a in soup.find_all('a',href=True):
    href=a['href']
    if href.startswith('#'):assert href[1:] in ids,href
assert len(soup.select('details.requirement'))==119
assert not soup.find_all('script',src=True)
assert not soup.find_all('link',href=True)
assert not soup.find_all('iframe')
source=soup.get_text()
assert '98件' in source and '93件' not in source
# Files with exact snippets/quoted intended error names remain syntactically executable.
python_files=[]
for folder in ('base','model','examples','tests','verification'):
    for p in (ROOT/folder).glob('*.py'):
        ast.parse(p.read_text(),filename=str(p));python_files.append(p.relative_to(ROOT).as_posix())
checks={'requirement_rows':119,'decision_rows':30,'theorem_rows':24,
    'master_approval_preserved':True,'missing_internal_links':0,
    'self_contained_runtime_assets':True,'render_mode':'set_content; file URL navigation disallowed by this browser policy','python_syntax_files':len(python_files),
    'mathematical_proofs_validated_by_this_script':False,'browser':[]}
try:
    from playwright.sync_api import sync_playwright
    with sync_playwright() as pw:
        browser=pw.chromium.launch(executable_path='/usr/bin/chromium',headless=True,args=['--no-sandbox'])
        for name,width,height in [('desktop',1400,1000),('mobile',390,844)]:
            page=browser.new_page(viewport={'width':width,'height':height},device_scale_factor=1)
            errors=[];page.on('pageerror',lambda e:errors.append(str(e)))
            page.set_content(html,wait_until='load')
            page.screenshot(path=str(ROOT/'evidence'/f'report_{name}.png'),full_page=False)
            overflow=page.evaluate('document.documentElement.scrollWidth > innerWidth')
            page.locator('#q').fill('ID-08');
            count=page.locator('details.requirement:not([hidden])').count();assert count==1,(name,count)
            page.locator('details.requirement:not([hidden]) summary').click()
            assert page.locator('#req-ID-08').evaluate('(e)=>e.open')
            page.locator('#q').fill('');page.locator('#group').select_option('AU');count_au=page.locator('details.requirement:not([hidden])').count();assert count_au==8
            page.locator('#group').select_option('');page.locator('#work').select_option('W5');count_w5=page.locator('details.requirement:not([hidden])').count();assert count_w5>0
            assert not errors,errors
            assert not overflow,(name,'horizontal overflow')
            checks['browser'].append({'viewport':name,'width':width,'height':height,'horizontal_overflow':overflow,'page_errors':errors,'exact_id_search_count':count,'auth_filter_count':count_au,'W5_filter_count':count_w5})
            page.close()
        browser.close()
except ImportError:
    checks['browser_not_executed']='Playwright unavailable'
(ROOT/'evidence/DOCUMENT_CHECKS.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(checks,ensure_ascii=False,indent=2))
