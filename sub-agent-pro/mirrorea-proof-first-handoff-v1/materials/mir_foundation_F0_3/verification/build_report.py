"""Render the self-contained research report (not needed for model execution)."""
from pathlib import Path
import re,json,html
import mistune
from bs4 import BeautifulSoup
ROOT=Path(__file__).resolve().parents[1]
markdown=mistune.create_markdown(escape=True,plugins=['table','strikethrough','url'])
parts=[]; headings=[]


def render(text,prefix):
    soup=BeautifulSoup(markdown(text),'html.parser')
    if prefix=='foundation' and soup.find('h1'):
        soup.find('h1').string='基礎の全体像と到達点'
    for idx,h in enumerate(soup.find_all(re.compile(r'^h[1-6]$'))):
        level=int(h.name[1]);h.name=f'h{min(6,level+1)}'
        ident=f'{prefix}-h{idx}'
        h['id']=ident
        if level<=2:headings.append((prefix,ident,h.get_text(),level))
    for table in soup.find_all('table'):
        wrapper=soup.new_tag('div',attrs={'class':'table-scroll','tabindex':'0'})
        table.wrap(wrapper)
    return str(soup)

for slug,title,file in [
 ('foundation','全体像と到達点','FOUNDATION.md'),
 ('proofs','定義と24の手証明','theory/PROOFS.md'),
 ('plan','実装への計画','WORKPLAN.md'),
 ('ledger','命題の範囲','theory/THEOREM_LEDGER.md'),
 ('audit','反例と修正','AUDIT.md'),
 ('validation','検査記録','VALIDATION.md'),
 ('decisions','30の未決事項','DECISION_DISPOSITIONS.md'),
 ('sources','出典と参照範囲','SOURCES.md'),
]:
    parts.append(f'<section id="{slug}" aria-label="{title}">{render((ROOT/file).read_text(),slug)}</section>')

requirements=json.loads((ROOT/'REQUIREMENT_TRACE.json').read_text())['requirements']
reqhtml=['<section id="requirements"><h2>119要件との対応</h2><p>どの行も、全要件の受理済み宣言ではありません。今回の限定的な根拠と、なお残る受入れ条件を並べています。</p><div class="filters"><label>検索<input id="q" type="search" placeholder="要件ID・DAG・観測・依存型など"></label><label>領域<select id="group"><option value="">全領域</option>'+''.join(f'<option>{html.escape(g)}</option>' for g in dict.fromkeys(r['id'].split('-')[0] for r in requirements))+'</select></label><label>作業単位<select id="work"><option value="">すべて</option>'+''.join(f'<option>W{i}</option>' for i in range(1,9))+'</select></label></div><p id="count" aria-live="polite"></p>']
for r in requirements:
    fields=[('元の要求',r['statement']),('今回の根拠',r['F0_3_result']),('未完の部分',r['remaining']),('正例の出口',r['acceptance_positive']),('反例の出口',r['acceptance_negative']),('関連する手証明','、'.join(r['theorems']) or 'この要件を解消する新規定理はありません。'),('次の作業',' / '.join(r['next_work_units']))]
    search=' '.join([r['id'],r['title']]+[v for _,v in fields])
    reqhtml.append(f'<details class="requirement" id="req-{r["id"]}" data-group="{r["id"].split("-")[0]}" data-work="{" ".join(r["next_work_units"])}" data-search="{html.escape(search,quote=True)}"><summary><span class="id">{r["id"]}</span> {html.escape(r["title"])} <span class="origin">{r["origin"]}</span></summary><dl>'+''.join(f'<dt>{k}</dt><dd>{html.escape(v)}</dd>' for k,v in fields)+'</dl></details>')
reqhtml.append('</section>')
parts.insert(4,''.join(reqhtml))

navitems=[('foundation','全体像'),('proofs','定義と証明'),('plan','実装計画'),('ledger','命題台帳'),('requirements','119要件'),('decisions','30の判断'),('audit','反例・修正'),('validation','検査記録'),('sources','出典')]
nav=''.join(f'<a href="#{slug}">{label}</a>' for slug,label in navitems)
body=''.join(parts)
# Source citations remain traceable within the standalone file.
body=re.sub(r'\[S([0-9]+)(?:,S([0-9]+))?\]',lambda m:f'<a class="source" href="#sources">{m.group(0)}</a>',body)
# Turn repository URLs in generated tables into safe source links automatically (mistune url plugin).
css='''
:root{font-family:system-ui,-apple-system,"Noto Sans CJK JP","Yu Gothic",sans-serif;color:#1d1d1d;background:#fff;font-size:16px;line-height:1.85}
*{box-sizing:border-box}html{scroll-behavior:smooth;scroll-padding-top:1rem}body{margin:0}a{color:inherit;text-decoration-thickness:1px;text-underline-offset:3px}a:hover{text-decoration-thickness:2px}
nav{position:fixed;inset:0 auto 0 0;width:208px;border-right:1px solid #ddd;padding:30px 20px;overflow:auto;background:#f8f8f8}nav .brand{font-size:18px;font-weight:700;margin-bottom:24px}nav a{display:block;font-size:14px;padding:8px 6px;text-decoration:none}nav a:hover{background:#eee}
main{margin-left:208px;max-width:1240px;padding:48px 56px 100px}header{padding-bottom:24px;border-bottom:2px solid #333}h1{font-size:31px;line-height:1.45;letter-spacing:.01em;margin:0 0 20px}h2{font-size:25px;line-height:1.5;margin-top:52px;padding-top:10px}h3{font-size:21px;line-height:1.5;margin-top:38px}h4{font-size:18px;line-height:1.6;margin-top:28px}p{margin:16px 0}section{padding:8px 0 30px;border-bottom:1px solid #ddd}section>h2:first-child{margin-top:30px}strong{font-weight:700}code,pre{font-family:ui-monospace,SFMono-Regular,Consolas,monospace;font-size:.88em}code{overflow-wrap:anywhere}pre{background:#f6f6f6;padding:18px;overflow:auto;line-height:1.65;border-left:3px solid #777}pre code{white-space:pre}blockquote{border-left:3px solid #888;margin:20px 0;padding:2px 18px;color:#444}table{width:100%;border-collapse:collapse;font-size:14px;line-height:1.65}th,td{padding:11px 13px;vertical-align:top;text-align:left;border-bottom:1px solid #ddd;min-width:100px;overflow-wrap:anywhere}th{background:#f3f3f3;font-weight:700}td:first-child{min-width:90px}tr:last-child td{border-bottom:1px solid #bbb}.table-scroll{overflow:auto;margin:24px 0;max-width:100%}a.source{font-size:.85em;color:#555;white-space:nowrap}.filters{display:flex;gap:16px;flex-wrap:wrap;align-items:flex-end}.filters label{font-size:13px;display:flex;flex-direction:column;gap:4px}.filters label:first-child{flex:1;min-width:210px}input,select{font:inherit;border:1px solid #aaa;border-radius:0;padding:9px 10px;background:#fff;color:#111;max-width:100%}#count{font-size:14px;color:#555}.requirement{border-top:1px solid #ddd;padding:10px 0}.requirement summary{cursor:pointer;font-weight:600;line-height:1.6;padding:8px 0}.id{display:inline-block;min-width:65px;font-family:ui-monospace,monospace;font-size:13px;color:#555}.origin{font-size:12px;font-weight:400;margin-left:8px;color:#666}dl{margin:8px 0 22px;padding-left:24px}dt{font-size:13px;font-weight:700;color:#555;margin:18px 0 4px}dd{margin:0}.requirement[hidden]{display:none}.note{font-size:14px;color:#555}.toclink{display:inline-block;margin-right:18px}
@media(max-width:800px){nav{position:static;width:auto;border-right:0;border-bottom:1px solid #ccc;padding:16px;display:flex;gap:7px;flex-wrap:wrap}nav .brand{width:100%;margin:0 0 6px}nav a{padding:4px 6px;font-size:13px}main{margin:0;padding:28px 20px 64px}h1{font-size:26px}h2{font-size:23px}h3{font-size:20px}.table-scroll{margin:18px 0}th,td{padding:9px 10px;min-width:145px}pre{padding:13px}dl{padding-left:12px}}
@media print{nav,.filters,#count{display:none}main{margin:0;max-width:none;padding:0}html{font-size:10pt}a{text-decoration:none}section{break-before:auto}h2,h3,h4{break-after:avoid}.requirement{display:block!important}.table-scroll,pre{overflow:visible}pre{white-space:pre-wrap}table{font-size:9pt}}
'''
js='''
const q=document.getElementById('q'),g=document.getElementById('group'),w=document.getElementById('work');
function filter(){let n=0;const term=q.value.trim().toLocaleLowerCase();document.querySelectorAll('.requirement').forEach(e=>{const ok=(!g.value||e.dataset.group===g.value)&&(!w.value||e.dataset.work.split(' ').includes(w.value))&&(!term||e.dataset.search.toLocaleLowerCase().includes(term));e.hidden=!ok;if(ok)n++;});document.getElementById('count').textContent=`表示 ${n} / 119要件（達成数ではありません）`;}
[q,g,w].forEach(e=>e.addEventListener('input',filter));filter();
window.addEventListener('hashchange',()=>{let e=document.getElementById(decodeURIComponent(location.hash.slice(1)));if(e&&e.tagName==='DETAILS'){e.hidden=false;e.open=true;}});
'''
header='''<header><h1>Mirroreaの基礎理論と実装への接続</h1><p>構成・証拠・現在性を結ぶF0.3研究候補。定義、手証明、実行モデル、反例と、要件に対応する実装計画を収録しています。</p><p class="note">24項目の手証明・98件の新規テスト。全文の証明支援系による検証、独立レビュー、実Rust／QUICへの対応は未完です。全119要件の受理やCanonの変更を表すものではありません。</p><p><a class="toclink" href="#foundation">到達点を読む</a><a class="toclink" href="#proofs">証明を読む</a><a class="toclink" href="#plan">実装の順序を読む</a></p></header>'''
page=f'<!doctype html><html lang="ja"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="color-scheme" content="light"><title>Mirrorea 基礎理論と実装への接続 F0.3</title><style>{css}</style></head><body><nav aria-label="目次"><div class="brand">Mirrorea · F0.3</div>{nav}</nav><main>{header}{body}</main><script>{js}</script></body></html>'
(ROOT/'REPORT.html').write_text(page,encoding='utf-8')
print('REPORT.html',len(page.encode()),'bytes',len(requirements),'requirements')
