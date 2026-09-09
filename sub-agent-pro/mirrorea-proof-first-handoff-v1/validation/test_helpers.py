"""Tests of packaging helpers only; no oracle or real external network is used."""
from pathlib import Path
import importlib.util
import json
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
ROOT=Path(__file__).resolve().parents[1]

def load(name, filename):
    spec=importlib.util.spec_from_file_location(name,ROOT/'tools'/filename)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
VERIFY=load('verify_handoff_test','verify_bundle.py')
LOGGER=load('logger_handoff_test','run_logged.py')

class Helpers(unittest.TestCase):
    def test_manifest_and_original_archives(self):
        r=VERIFY.verify(ROOT);self.assertTrue(r['passed'],r);self.assertEqual(r['requirements'],119)
    def test_unsafe_relative_paths_rejected(self):
        for name in ['../x','/tmp/x','a/../../x','a\\b','x\x00y']:
            with self.subTest(name=name):
                with self.assertRaises(ValueError):VERIFY.safe_relative(name)
    def test_content_tamper_detected(self):
        with tempfile.TemporaryDirectory() as td:
            copy=Path(td)/'copy';shutil.copytree(ROOT,copy,ignore=shutil.ignore_patterns('__pycache__'))
            (copy/'context/OWNER_INTENT.md').write_text('changed')
            result=VERIFY.verify(copy);self.assertFalse(result['passed']);self.assertIn('content mismatch: context/OWNER_INTENT.md',result['failures'])
    def test_prompt_copy_exact(self):
        info=json.loads((ROOT/'provenance/PROMPT.json').read_text());self.assertEqual(VERIFY.digest((ROOT/'RUN_CODEX_MIRROREA.md').read_bytes()),info['sha256'])
    def test_mismatched_separate_prompt_rejected(self):
        with tempfile.TemporaryDirectory() as td:
            q=Path(td)/'prompt.md';q.write_text('not the matched prompt')
            p=subprocess.run([sys.executable,str(ROOT/'tools/verify_bundle.py'),'--prompt-file',str(q)],capture_output=True,text=True)
            self.assertEqual(p.returncode,1);self.assertIn('separate prompt file differs',p.stdout)
    def test_run_logged_success_and_exact_argv(self):
        with tempfile.TemporaryDirectory() as td:
            out=Path(td)/'run'
            p=subprocess.run([sys.executable,str(ROOT/'tools/run_logged.py'),'--directory',str(out),'--cwd',td,'--',sys.executable,'-c','import sys; print(sys.argv[1])','literal $() ; spaces'],capture_output=True,text=True)
            self.assertEqual(p.returncode,0,p.stderr);self.assertEqual((out/'output.log').read_text().strip(),'literal $() ; spaces')
            meta=json.loads((out/'process.json').read_text());self.assertEqual(meta['status'],'process_exited');self.assertIsNone(meta['walltime_limit'])
    def test_run_logged_preserves_nonzero(self):
        with tempfile.TemporaryDirectory() as td:
            out=Path(td)/'run'
            p=subprocess.run([sys.executable,str(ROOT/'tools/run_logged.py'),'--directory',str(out),'--cwd',td,'--',sys.executable,'-c','raise SystemExit(7)'],capture_output=True,text=True)
            self.assertEqual(p.returncode,7);self.assertEqual(json.loads((out/'process.json').read_text())['exit_code'],7)
    def test_low_poll_interval_rejected(self):
        with tempfile.TemporaryDirectory() as td:
            out=Path(td)/'run'
            p=subprocess.run([sys.executable,str(ROOT/'tools/run_logged.py'),'--directory',str(out),'--cwd',td,'--interval-seconds','10','--','anything'],capture_output=True,text=True)
            self.assertEqual(p.returncode,2);self.assertFalse(out.exists())
    def test_logging_poll_uses_180_seconds_without_real_wait(self):
        class Fake:
            pid=987654
            def __init__(self):self.calls=[]
            def wait(self,timeout):
                self.calls.append(timeout)
                if len(self.calls)==1:raise subprocess.TimeoutExpired('fake',timeout)
                return 0
        with tempfile.TemporaryDirectory() as td:
            fake=Fake();out=Path(td)/'run'
            args=['run_logged.py','--directory',str(out),'--cwd',td,'--','fake']
            with patch.object(sys,'argv',args),patch.object(LOGGER.subprocess,'Popen',return_value=fake):
                self.assertEqual(LOGGER.main(),0)
            self.assertEqual(fake.calls,[180,180])
    def test_packet_selects_exact_files_without_submission(self):
        with tempfile.TemporaryDirectory() as td:
            td=Path(td);repo=td/'repo';repo.mkdir();subprocess.run(['git','init','-q',str(repo)],check=True)
            (repo/'a.md').write_text('original');subprocess.run(['git','-C',str(repo),'add','a.md'],check=True)
            subprocess.run(['git','-C',str(repo),'-c','user.name=Helper Test','-c','user.email=helper@example.invalid','commit','--no-gpg-sign','-qm','base'],check=True)
            (repo/'a.md').write_text('selected dirty bytes');question=td/'question.md';question.write_text('Review the exact selected bytes.');out=td/'packet'
            p=subprocess.run([sys.executable,str(ROOT/'tools/make_oracle_packet.py'),'--repo',str(repo),'--question',str(question),'--output',str(out),'--file','a.md'],capture_output=True,text=True)
            self.assertEqual(p.returncode,0,p.stderr);self.assertFalse(json.loads(p.stdout)['submitted']);self.assertEqual((out/'files/a.md').read_text(),'selected dirty bytes')
            self.assertEqual(len(json.loads((out/'PACKET_MANIFEST.json').read_text())['files']),1)
    def test_secret_name_refused(self):
        with tempfile.TemporaryDirectory() as td:
            td=Path(td);repo=td/'repo';repo.mkdir();(repo/'.env').write_text('not-a-real-secret');q=td/'q.md';q.write_text('Review');out=td/'packet'
            p=subprocess.run([sys.executable,str(ROOT/'tools/make_oracle_packet.py'),'--repo',str(repo),'--question',str(q),'--output',str(out),'--file','.env'],capture_output=True,text=True)
            self.assertEqual(p.returncode,2);self.assertFalse(out.exists())
    def test_baseline_cannot_overwrite_existing(self):
        with tempfile.TemporaryDirectory() as td:
            td=Path(td);work=td/'work';work.mkdir();evi=td/'ev'
            p=subprocess.run([sys.executable,str(ROOT/'tools/run_baselines.py'),'--work-root',str(work),'--evidence-root',str(evi)],capture_output=True,text=True)
            self.assertEqual(p.returncode,2);self.assertFalse(evi.exists())
    def test_baseline_cannot_write_inside_original_bundle(self):
        with tempfile.TemporaryDirectory() as td:
            p=subprocess.run([sys.executable,str(ROOT/'tools/run_baselines.py'),'--work-root',str(ROOT/'materials/do-not-create'),'--evidence-root',str(Path(td)/'ev')],capture_output=True,text=True)
            self.assertEqual(p.returncode,2);self.assertFalse((ROOT/'materials/do-not-create').exists())
if __name__=='__main__':unittest.main()
