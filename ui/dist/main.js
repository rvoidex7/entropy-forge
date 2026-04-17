"use strict";(()=>{var Wt=(a,r)=>()=>(a&&(r=a(a=0)),r);var Jt=(a,r)=>()=>(r||a((r={exports:{}}).exports,r),r.exports);var I=(a,r,l)=>new Promise((p,u)=>{var k=v=>{try{w(l.next(v))}catch(_){u(_)}},z=v=>{try{w(l.throw(v))}catch(_){u(_)}},w=v=>v.done?p(v.value):Promise.resolve(v.value).then(k,z);w((l=l.apply(a,r)).next())});function b(a,r,l,p){if(l==="a"&&!p)throw new TypeError("Private accessor was defined without a getter");if(typeof r=="function"?a!==r||!p:!r.has(a))throw new TypeError("Cannot read private member from an object whose class did not declare it");return l==="m"?p:l==="a"?p.call(a):p?p.value:r.get(a)}function P(a,r,l,p,u){if(p==="m")throw new TypeError("Private method is not writable");if(p==="a"&&!u)throw new TypeError("Private accessor was defined without a setter");if(typeof r=="function"?a!==r||!u:!r.has(a))throw new TypeError("Cannot write private member to an object whose class did not declare it");return p==="a"?u.call(a,l):u?u.value=l:r.set(a,l),l}var jt=Wt(()=>{});function te(a,r=!1){return window.__TAURI_INTERNALS__.transformCallback(a,r)}function zt(p){return I(this,arguments,function*(a,r={},l){return window.__TAURI_INTERNALS__.invoke(a,r,l)})}var $,g,A,et,Qt,Ot,Dt,Xt=Wt(()=>{jt();Ot="__TAURI_TO_IPC_KEY__";Dt=class{constructor(r){$.set(this,void 0),g.set(this,0),A.set(this,[]),et.set(this,void 0),P(this,$,r||(()=>{}),"f"),this.id=te(l=>{let p=l.index;if("end"in l){p==b(this,g,"f")?this.cleanupCallback():P(this,et,p,"f");return}let u=l.message;if(p==b(this,g,"f")){for(b(this,$,"f").call(this,u),P(this,g,b(this,g,"f")+1,"f");b(this,g,"f")in b(this,A,"f");){let k=b(this,A,"f")[b(this,g,"f")];b(this,$,"f").call(this,k),delete b(this,A,"f")[b(this,g,"f")],P(this,g,b(this,g,"f")+1,"f")}b(this,g,"f")===b(this,et,"f")&&this.cleanupCallback()}else b(this,A,"f")[p]=u})}cleanupCallback(){window.__TAURI_INTERNALS__.unregisterCallback(this.id)}set onmessage(r){P(this,$,r,"f")}get onmessage(){return b(this,$,"f")}[($=new WeakMap,g=new WeakMap,A=new WeakMap,et=new WeakMap,Ot)](){return`__CHANNEL__:${this.id}`}toJSON(){return this[Ot]()}};Qt=new WeakMap});var ee=Jt(D=>{Xt();var B=(l,...p)=>I(D,[l,...p],function*(a,r={}){if(window.__TAURI_INTERNALS__)return yield zt(a,r);throw console.warn(`[Tauri Mock] Called invoke('${a}') with args:`,r),new Error("Tauri API is not available in the browser.")});document.addEventListener("DOMContentLoaded",()=>{let a=document.querySelectorAll(".tab-link, .mobile-tab-link"),r=document.querySelectorAll(".tab-panel");function l(t){r.forEach(e=>{e.classList.remove("active")});let n=document.getElementById(t);n&&n.classList.add("active"),document.querySelectorAll(".tab-link").forEach(e=>{if(e.getAttribute("data-target")===t){e.classList.remove("text-slate-500"),e.classList.add("text-[#00FF88]","bg-[#00FF88]/10","border-l-2","border-[#00FF88]");let s=e.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 1")}else{e.classList.add("text-slate-500"),e.classList.remove("text-[#00FF88]","bg-[#00FF88]/10","border-l-2","border-[#00FF88]");let s=e.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 0")}}),document.querySelectorAll(".mobile-tab-link").forEach(e=>{if(e.getAttribute("data-target")===t){e.classList.remove("text-gray-500"),e.classList.add("text-[#00FF88]","border-t-2","border-[#00FF88]");let s=e.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 1")}else{e.classList.add("text-gray-500"),e.classList.remove("text-[#00FF88]","border-t-2","border-[#00FF88]");let s=e.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 0")}})}a.forEach(t=>{t.addEventListener("click",n=>{n.preventDefault();let e=n.currentTarget.getAttribute("data-target");e&&l(e)})}),document.querySelectorAll(".sub-tab-link").forEach(t=>{t.addEventListener("click",n=>{n.preventDefault(),l("tab-learn");let e=n.currentTarget.getAttribute("data-target");if(e){let s=document.getElementById(e);s&&setTimeout(()=>{s.scrollIntoView({behavior:"smooth",block:"start"})},100)}})});let u=document.getElementById("use-input"),k=document.getElementById("use-input-length"),z=document.getElementById("use-hex-check"),w=document.getElementById("use-output"),v=document.getElementById("use-encrypt-btn"),_=document.getElementById("use-keystream-grid");u&&k&&u.addEventListener("input",()=>{k.textContent=`LENGTH: ${u.value.length} BYTES`}),v&&u&&w&&z&&_&&v.addEventListener("click",()=>I(D,null,function*(){let t=u.value;if(t)try{let n=yield B("encrypt_decrypt",{plaintext:t,hexOutput:z.checked});w.value=n.ciphertext,_.innerHTML="";let e=n.keystream_bytes||[],s=t.length,o=_.parentElement;if(o){let i=o.clientWidth,c=20,d=4,y=i-16,f=c+d,mt=Math.floor(y/f),bt=Math.max(1,mt);_.style.gridTemplateColumns=`repeat(${bt}, 20px)`}for(let i=0;i<s;i++){let c=document.createElement("div"),d=e[i]||0,m=Math.round(d/255*255),y=`rgb(${m}, ${m}, ${m})`;c.style.backgroundColor=y,c.title=`Byte ${i}: 0x${d.toString(16).toUpperCase().padStart(2,"0")} (${d})`,_.appendChild(c)}}catch(n){console.error(n),w.value=`Error: ${n}`}}));let yt=document.getElementById("use-help-link");yt&&yt.addEventListener("click",()=>{l("tab-learn");let t=document.getElementById("learn-input");t&&u&&u.value&&(t.value=u.value,t.dispatchEvent(new Event("input")))});let q=document.getElementById("test-size-slider"),xt=document.getElementById("test-size-label"),ft=document.getElementById("test-run-btn"),gt=document.getElementById("test-results-section");q&&xt&&q.addEventListener("input",()=>{let t=parseFloat(q.value),n=Math.round(Math.pow(10,t));xt.textContent=`${n.toLocaleString()} BYTES`}),ft&&q&&gt&&ft.addEventListener("click",()=>I(D,null,function*(){let t=parseFloat(q.value),n=Math.round(Math.pow(10,t));gt.classList.remove("hidden");let e=document.getElementById("test-nist-tbody");e&&(e.innerHTML='<tr><td colspan="3" class="text-center py-4">Running tests...</td></tr>');try{let s=yield B("run_quality_tests",{sampleSize:n});document.getElementById("test-shannon-val").textContent=s.shannon_entropy.toFixed(4),document.getElementById("test-shannon-bar").style.width=`${s.shannon_entropy/8*100}%`,document.getElementById("test-min-val").textContent=s.min_entropy.toFixed(4),document.getElementById("test-min-bar").style.width=`${s.min_entropy/8*100}%`,document.getElementById("test-mean-val").textContent=s.mean.toFixed(3),document.getElementById("test-chi-val").textContent=s.chi_square.toFixed(2),document.getElementById("test-longest-run-val").textContent=s.longest_run.toString(),document.getElementById("test-score-val").textContent=s.overall_score.toFixed(1),e&&(e.innerHTML="",s.nist_results.forEach(o=>{let i=document.createElement("tr");i.className="border-b border-outline-variant/10 hover:bg-surface-container-low transition-colors group";let c=o.passed?'<span class="bg-[#00FF88]/10 text-[#00FF88] border border-[#00FF88]/30 px-3 py-1 font-bold">\u2713 PASS</span>':'<span class="bg-error-container/20 text-error border border-error/30 px-3 py-1 font-bold">\u2717 FAIL</span>';i.innerHTML=`
                            <td class="px-6 py-4 font-bold uppercase tracking-tight">${o.name}</td>
                            <td class="px-6 py-4 font-mono">${o.p_value.toFixed(6)}</td>
                            <td class="px-6 py-4 text-right">${c}</td>
                        `,e.appendChild(i)}))}catch(s){console.error(s),e&&(e.innerHTML=`<tr><td colspan="3" class="text-center text-error py-4">Error: ${s}</td></tr>`)}}));let M=document.getElementById("bench-size-slider"),X=document.getElementById("bench-size-label"),vt=document.getElementById("bench-run-btn"),ht=document.getElementById("bench-results-section");M&&X&&(M.addEventListener("input",()=>{let t=parseFloat(M.value),n=Math.round(Math.pow(10,t));n>=1e6?X.textContent=`${(n/1e6).toFixed(1)}M bytes`:n>=1e3?X.textContent=`${(n/1e3).toFixed(0)}K bytes`:X.textContent=`${n} bytes`}),M.dispatchEvent(new Event("input"))),vt&&M&&ht&&vt.addEventListener("click",()=>I(D,null,function*(){let t=parseFloat(M.value),n=Math.round(Math.pow(10,t));ht.classList.remove("hidden");try{let e=yield B("run_benchmark",{bytes:n});document.getElementById("bench-throughput").textContent=e.throughput_mbps.toFixed(2),document.getElementById("bench-latency").textContent=e.latency_us.toFixed(2);let s=e.bytes_generated.toLocaleString();document.getElementById("bench-details").textContent=`Generated ${s} bytes in ${e.duration_secs.toFixed(3)} seconds`}catch(e){console.error(e),document.getElementById("bench-details").textContent=`Error: ${e}`}}));let Et=document.querySelectorAll(".learn-btn"),Ut=document.querySelectorAll(".learn-section");Et.forEach(t=>{t.addEventListener("click",n=>{let e=n.currentTarget.getAttribute("data-learn");Et.forEach(s=>{s.classList.remove("active","bg-primary-container","text-on-primary-container"),s.classList.add("bg-surface-container","text-slate-400")}),n.currentTarget.classList.remove("bg-surface-container","text-slate-400"),n.currentTarget.classList.add("active","bg-primary-container","text-on-primary-container"),Ut.forEach(s=>{s.id===`learn-${e}`?s.classList.add("active"):s.classList.remove("active")})})});let h=[],x=0,S=null,U=document.getElementById("learn-input"),It=document.getElementById("learn-prev"),V=document.getElementById("learn-next"),G=document.getElementById("learn-play-pause"),K=document.getElementById("learn-speed-slider"),_t=document.getElementById("learn-speed-text"),Lt=document.getElementById("learn-xor-step-info"),Z=document.getElementById("learn-xor-content"),wt=document.getElementById("learn-progress-text"),St=document.getElementById("learn-progress-slider");function Ct(){return I(this,null,function*(){if(U.value)try{h=(yield B("get_xor_steps",{text:U.value})).steps,x=0,S&&N(),nt()}catch(t){console.error(t)}})}function nt(){if(h.length===0){Z&&(Z.innerHTML="No steps.");return}let t=h[x];Lt&&(Lt.textContent=`STEP_IDX: ${(x+1).toString().padStart(3,"0")} / ${h.length.toString().padStart(3,"0")}`);let n=Math.round((x+1)/h.length*100);if(wt&&(wt.textContent=`${n}%`),St&&(St.value=n.toString()),Z){let e=t.input_binary.split("").map(i=>`<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-cyan-400">${i}</span>`).join(""),s=t.keystream_binary.split("").map(i=>`<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-orange-400">${i}</span>`).join(""),o=t.result_binary.split("").map(i=>`<span class="w-6 h-8 bg-primary-container/10 border border-green-400/30 flex items-center justify-center font-black text-green-400">${i}</span>`).join("");Z.innerHTML=`
             <div class="space-y-6">
                 <h3 class="text-lg font-bold text-primary">Step ${x+1} of ${h.length}: Encrypting '${t.character}'</h3>
                 
                 <div class="bg-surface-container-low p-4 relative overflow-hidden border-l-2 border-[#00FF88] rounded">
                     <div class="flex justify-between items-start mb-4">
                         <div>
                             <span class="text-[10px] text-secondary uppercase font-bold">Input Character</span>
                             <div class="text-4xl font-black text-primary">'${t.character}'</div>
                         </div>
                         <div class="text-right">
                             <span class="text-[10px] text-outline-variant uppercase">ASCII Decimal</span>
                             <div class="text-xl font-bold">${t.input_byte}</div>
                         </div>
                     </div>

                     <div class="space-y-4 pt-4 border-t border-outline-variant/30">
                         <div class="flex items-center justify-between">
                             <span class="text-[10px] text-cyan-400 text-left uppercase w-20 font-bold">Plaintext</span>
                             <div class="flex gap-1">${e}</div>
                         </div>

                         <div class="flex items-center gap-4">
                             <div class="h-[1px] flex-grow bg-outline-variant"></div>
                             <span class="text-[#A855F7] font-black italic">\u2295 XOR</span>
                             <div class="h-[1px] flex-grow bg-outline-variant"></div>
                         </div>

                         <div class="flex items-center justify-between">
                             <span class="text-[10px] text-orange-400 text-left uppercase w-20 font-bold">Keystream</span>
                             <div class="flex gap-1">${s}</div>
                         </div>

                         <div class="flex items-center justify-between pt-4 border-t-2 border-green-400/30">
                             <span class="text-[10px] text-green-400 text-left uppercase w-20 font-bold">Result</span>
                             <div class="flex gap-1">${o}</div>
                         </div>
                     </div>
                 </div>
             </div>
             `}Vt()}function Vt(){let t=document.getElementById("learn-xor-progress");if(!t)return;if(h.length===0){t.classList.add("hidden");return}t.classList.remove("hidden");let n=t.querySelector(".flex.flex-wrap.gap-2");if(!n)return;let e="";h.forEach((s,o)=>{let i=s.result_byte.toString(16).toUpperCase().padStart(2,"0"),c="px-3 py-2 rounded font-mono text-sm font-bold transition-colors ";o===x?c+="bg-green-500/30 border border-green-400 text-green-300":o<x?c+="bg-surface-container-highest text-slate-400 border border-outline-variant/20":c+="bg-surface-container text-slate-500 border border-outline-variant/20",e+=`<span class="${c}">${i}</span>`}),n.innerHTML=e}U&&(U.addEventListener("input",()=>{Ct()}),setTimeout(Ct,500)),It&&It.addEventListener("click",()=>{x>0&&(x--,nt())}),V&&V.addEventListener("click",()=>{x<h.length-1?(x++,nt()):S&&N()});function N(){if(!G)return;let t=G.querySelector(".material-symbols-outlined");if(S)clearInterval(S),S=null,t&&(t.textContent="play_arrow");else{x>=h.length-1&&(x=0);let n=parseInt(K.value,10);S=setInterval(()=>{V&&V.click()},n),t&&(t.textContent="pause")}}G&&G.addEventListener("click",N),K&&_t&&K.addEventListener("input",()=>{let t=(parseInt(K.value,10)/1e3).toFixed(1);_t.textContent=`${t}s / step`,S&&(N(),N())});let st=document.getElementById("learn-xor-collapsible"),rt=document.getElementById("learn-xor-details");st&&rt&&st.addEventListener("click",()=>{rt.classList.toggle("hidden");let t=st.querySelector(".material-symbols-outlined");t&&(t.textContent=rt.classList.contains("hidden")?"expand_more":"expand_less")});let at=document.getElementById("learn-entropy-collapsible"),ot=document.getElementById("learn-entropy-details");at&&ot&&at.addEventListener("click",()=>{ot.classList.toggle("hidden");let t=at.querySelector(".material-symbols-outlined");t&&(t.textContent=ot.classList.contains("hidden")?"expand_more":"expand_less")});let it=document.getElementById("learn-nist-collapsible"),lt=document.getElementById("learn-nist-details");it&&lt&&it.addEventListener("click",()=>{lt.classList.toggle("hidden");let t=it.querySelector(".material-symbols-outlined");t&&(t.textContent=lt.classList.contains("hidden")?"expand_more":"expand_less")});let H=[],L=0,C=null,Tt=document.getElementById("learn-entropy-input"),Ft=document.getElementById("learn-entropy-prev"),Y=document.getElementById("learn-entropy-next"),R=document.getElementById("learn-entropy-play-pause"),$t=document.getElementById("learn-entropy-step-info"),Bt=document.getElementById("learn-entropy-content"),kt=document.getElementById("learn-entropy-calculate"),J=document.getElementById("learn-entropy-speed-slider"),Mt=document.getElementById("learn-entropy-speed-text");function Gt(){return I(this,null,function*(){if(Tt.value)try{H=(yield B("get_entropy_steps",{text:Tt.value})).steps,L=0,C&&W(),ct()}catch(t){console.error(t)}})}function ct(){var e,s;if(H.length===0)return;let t=H[L],n=t.step_type;if($t&&($t.textContent=`Step ${L+1} / ${H.length}`),Bt){let o=`<table class="w-full text-sm">
                 <thead>
                     <tr class="border-b border-outline-variant/30 text-left">
                         <th class="p-2 font-bold text-primary">Byte</th>
                         <th class="p-2 font-bold text-primary">Count</th>`;n!=="CountBytes"&&(o+='<th class="p-2 font-bold text-primary">Probability</th>'),(n==="CalculateContributions"||n==="SumEntropy"||n==="Interpret")&&(o+='<th class="p-2 font-bold text-primary">Contribution</th>'),o+=`<th class="p-2 font-bold text-primary">Visual</th>
                     </tr>
                 </thead>
                 <tbody>`;let i=Object.keys(t.byte_counts).sort(),c=Math.max(...Object.values(t.byte_counts),1);for(let m of i){let y=parseInt(m),f=t.byte_counts[m],mt=((e=t.probabilities)==null?void 0:e[m])||0,bt=((s=t.entropy_contributions)==null?void 0:s[m])||0,Kt=y>=32&&y<=126?String.fromCharCode(y):`0x${y.toString(16).toUpperCase().padStart(2,"0")}`,Zt=f/c*100,Yt=f===1?"#00FF88":`rgb(255, ${100+(155-Math.min(f*20,155))}, 100)`;o+=`<tr class="border-b border-outline-variant/20 hover:bg-surface-container-low transition-colors">
                     <td class="p-2 font-mono text-slate-300">'${Kt}'</td>
                     <td class="p-2 font-mono text-slate-300">${f}</td>`,n!=="CountBytes"&&(o+=`<td class="p-2 font-mono text-slate-300">${mt.toFixed(3)}</td>`),(n==="CalculateContributions"||n==="SumEntropy"||n==="Interpret")&&(o+=`<td class="p-2 font-mono text-slate-300">${bt.toFixed(3)} bits</td>`),o+=`<td class="p-2"><div class="h-6 bg-surface-container rounded" style="width: ${Zt}%; background-color: ${Yt};"></div></td>
                     </tr>`}o+="</tbody></table>";let d="";switch(n){case"CountBytes":d=`<div class="mt-6 p-4 bg-surface-container-low rounded border-l-2 border-primary">
                         <p class="text-sm text-slate-300">
                             <strong>\u{1F4A1} What's happening:</strong> We count how many times each unique byte appears in the input. This frequency distribution is the foundation for entropy calculation.
                         </p>
                     </div>`;break;case"CalculateProbabilities":d=`<div class="mt-6 space-y-3">
                         <div class="p-4 bg-surface-container-low rounded border-l-2 border-primary">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>\u{1F4A1} Formula:</strong> P(x) = Count(x) \xF7 Total Bytes
                             </p>
                             <p class="text-sm text-slate-300">
                                 This tells us the likelihood of each byte appearing in the input. Higher probability = more common byte.
                             </p>
                         </div>
                         <button class="w-full p-2 bg-surface-container rounded text-xs text-slate-400 hover:bg-surface-container-highest transition-colors text-left">
                             \u25B6 Learn More: Why does this matter?
                         </button>
                     </div>`;break;case"CalculateContributions":d=`<div class="mt-6 space-y-3">
                         <div class="p-4 bg-surface-container-low rounded border-l-2 border-primary">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>\u{1F4A1} Formula:</strong> Contribution = -P(x) \xD7 log\u2082(P(x))
                             </p>
                             <p class="text-sm text-slate-300">
                                 Rare events provide MORE information when they occur. A common byte (50%) is less surprising than a rare one (5%).
                             </p>
                         </div>
                         <button class="w-full p-2 bg-surface-container rounded text-xs text-slate-400 hover:bg-surface-container-highest transition-colors text-left">
                             \u25B6 Learn More: Information Theory Background
                         </button>
                     </div>`;break;case"SumEntropy":case"Interpret":{let m=t.max_entropy>0?t.total_entropy/t.max_entropy*100:0,y="Moderate entropy.",f="\u26A0\uFE0F";t.current_entropy_sum<2?(y="Low entropy. The input is very predictable.",f="\u{1F4C9}"):m>80&&(y="High entropy! The input looks quite random or uses many different characters.",f="\u{1F525}"),d=`<div class="mt-6 space-y-4">
                          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                              <div class="bg-surface-container-low p-4 rounded border border-primary/30">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Total Entropy</div>
                                  <div class="text-2xl font-bold text-primary">${t.current_entropy_sum.toFixed(4)}</div>
                                  <div class="text-xs text-slate-500 mt-1">bits/byte</div>
                              </div>
                              <div class="bg-surface-container-low p-4 rounded border border-primary/30">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Maximum Possible</div>
                                  <div class="text-2xl font-bold text-primary">${t.max_entropy.toFixed(4)}</div>
                                  <div class="text-xs text-slate-500 mt-1">bits/byte (${i.length} unique)</div>
                              </div>
                              <div class="bg-surface-container-low p-4 rounded border border-[#A855F7]/30">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Efficiency</div>
                                  <div class="text-2xl font-bold text-[#A855F7]">${m.toFixed(1)}%</div>
                                  <div class="text-xs text-slate-500 mt-1">how close to max</div>
                              </div>
                          </div>

                          <div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                              <p class="text-sm text-slate-300 mb-2">
                                  <strong>${f} Interpretation:</strong>
                              </p>
                              <p class="text-sm text-slate-300">
                                  ${y}
                              </p>
                              ${t.current_entropy_sum<2?'<p class="text-sm text-slate-400 mt-2 italic">For cryptography, you generally want entropy > 7 bits/byte for strong randomness.</p>':""}
                          </div>
                      </div>`;break}}Bt.innerHTML=o+d}}function W(){if(!R)return;let t=R.querySelector(".material-symbols-outlined"),n=R.querySelector("span:last-child");if(C)clearInterval(C),C=null,t&&(t.textContent="play_arrow"),n&&(n.textContent="Play");else{L>=H.length-1&&(L=0);let e=parseInt(J.value,10);C=setInterval(()=>{Y&&Y.click()},e),t&&(t.textContent="pause"),n&&(n.textContent="Pause")}}kt&&kt.addEventListener("click",Gt),Ft&&Ft.addEventListener("click",()=>{L>0&&(L--,ct())}),Y&&Y.addEventListener("click",()=>{L<H.length-1?(L++,ct()):C&&W()}),R&&R.addEventListener("click",W),J&&Mt&&J.addEventListener("input",()=>{let t=(parseInt(J.value,10)/1e3).toFixed(1);Mt.textContent=`${t}s / step`,C&&(W(),W())});let T=[],E=0,F=null,dt=document.getElementById("learn-nist-input"),Ht=document.getElementById("learn-nist-analyze"),Pt=document.getElementById("learn-nist-random"),At=document.getElementById("learn-nist-prev"),Q=document.getElementById("learn-nist-next"),j=document.getElementById("learn-nist-play-pause"),qt=document.getElementById("learn-nist-step-info"),Nt=document.getElementById("learn-nist-content"),tt=document.getElementById("learn-nist-speed-slider"),Rt=document.getElementById("learn-nist-speed-text");function ut(t=!0,n=""){return I(this,null,function*(){try{let e;t?e=yield B("get_nist_steps_random",{count:64}):e=yield B("get_nist_steps",{text:n}),T=e.steps,E=0,F&&O(),pt()}catch(e){console.error(e)}})}function pt(){if(T.length===0)return;let t=T[E],n=t.step_type;if(qt&&(qt.textContent=`Step ${E+1} / ${T.length}`),Nt){let e="";for(let d=0;d<t.bits.length;d++){d>0&&d%8===0&&(e+='<span class="mx-2"></span>');let m=t.bits[d];e+=`<span class="inline-block w-4 text-center font-bold ${m===1?"text-[#00FF88]":"text-slate-400"}">${m}</span>`}let s=`<div class="space-y-6">
                 <h3 class="text-lg font-bold text-[#A855F7]">Step ${E+1} of ${T.length}: ${n.replace(/([A-Z])/g," $1").trim()}</h3>

                 <div>
                     <h4 class="text-sm font-bold text-slate-300 mb-2">Bit Sequence (${t.bits.length} bits):</h4>
                     <div class="bg-surface-container-lowest p-4 rounded border border-outline-variant/20 font-mono text-sm break-all">
                         ${e}
                     </div>
                 </div>`,o=t.bits.length,i=o>0?t.ones_count/o*100:0;n!=="ConvertToBits"&&(s+=`<div class="grid grid-cols-2 gap-4">
                     <div class="bg-surface-container-low p-4 rounded border border-outline-variant/20">
                         <div class="text-xs text-slate-400 uppercase font-bold mb-1">Ones Count</div>
                         <div class="text-3xl font-bold text-[#00FF88]">${t.ones_count}</div>
                     </div>
                     <div class="bg-surface-container-low p-4 rounded border border-outline-variant/20">
                         <div class="text-xs text-slate-400 uppercase font-bold mb-1">Zeros Count</div>
                         <div class="text-3xl font-bold text-slate-400">${t.zeros_count}</div>
                     </div>
                 </div>

                 <!-- Balance Visualization -->
                 <div>
                     <h4 class="text-sm font-bold text-slate-300 mb-2">Distribution Balance (Ideal: 50% Ones)</h4>
                     <div class="relative h-12 bg-slate-600 rounded overflow-hidden">
                         <div class="absolute h-full bg-[#00FF88] transition-all" style="width: ${i}%;"></div>
                         <div class="absolute top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2 w-0.5 h-full bg-black"></div>
                         <div class="absolute inset-0 flex items-center justify-center font-bold text-sm text-white mix-blend-multiply">
                             ${i.toFixed(1)}% Ones (${t.ones_count}/${o} bits)
                         </div>
                     </div>
                 </div>`);let c="";switch(n){case"ConvertToBits":c=`<div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                         <p class="text-sm text-slate-300">
                             <strong>\u{1F4A1} What's happening:</strong> Each byte becomes 8 binary digits. White = 0, Green = 1.
                         </p>
                     </div>`;break;case"CountOnesZeros":c=`<div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                         <p class="text-sm text-slate-300 mb-2">
                             <strong>\u{1F4A1} What's happening:</strong> If the data is truly random, we expect roughly 50% ones and 50% zeros, like fair coin flips.
                         </p>
                         <p class="text-sm text-slate-400 italic">
                             Current ratio: ${i.toFixed(1)}% ones vs ${(100-i).toFixed(1)}% zeros
                         </p>
                     </div>`;break;case"CalculateStatistic":c=`<div class="space-y-3">
                         <div class="bg-surface-container-low p-4 rounded border-l-2 border-[#A855F7]">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>\u{1F4A1} Formula:</strong> S_obs = |Sum| \xF7 \u221A(n)
                             </p>
                             <p class="text-sm text-slate-400">Where Sum = (count_ones \xD7 +1) + (count_zeros \xD7 -1)</p>
                             <p class="text-sm text-slate-400">Result: ${t.sum} \xF7 \u221A${o} = ${t.s_obs.toFixed(4)}</p>
                         </div>
                         <div class="p-3 bg-surface-container rounded text-xs text-slate-400">
                             <p>S_obs tells us how far from balanced (0) the sequence is. Closer to 0 = more balanced and random-looking.</p>
                         </div>
                     </div>`;break;case"CalculatePValue":c=`<div class="space-y-3">
                         <div class="bg-surface-container-low p-4 rounded border-l-2 border-[#A855F7]">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>\u{1F4A1} Formula:</strong> P-value = erfc(S_obs \xF7 \u221A2)
                             </p>
                             <p class="text-sm text-slate-400">Result: erfc(${t.s_obs.toFixed(4)} \xF7 1.414) = ${t.p_value.toFixed(4)}</p>
                         </div>
                         <div class="p-3 bg-surface-container rounded text-xs text-slate-400">
                             <p><strong>Interpretation:</strong> P-value = ${(t.p_value*100).toFixed(1)}% probability that a truly random sequence would show this much deviation.</p>
                         </div>
                     </div>`;break;case"Interpret":{let d=t.passed?"#00FF88":"#ff6b6b",m=t.passed?"\u2705":"\u274C",y=t.passed?"PASS":"FAIL",f=t.passed?"The sequence looks RANDOM!":"The sequence is BIASED toward 1s or 0s.";c=`<div class="space-y-4">
                          <div class="p-4 rounded border-l-4" style="border-color: ${d}; background: rgba(0,0,0,0.2);">
                              <div class="flex items-center gap-2 mb-2">
                                  <span style="color: ${d}" class="text-2xl font-bold">${m} ${y}</span>
                              </div>
                              <p class="text-lg text-slate-300 font-bold mb-2">${f}</p>
                          </div>

                          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                              <div class="bg-surface-container-low p-4 rounded">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Test Statistic (S_obs)</div>
                                  <div class="text-2xl font-bold text-slate-300">${t.s_obs.toFixed(4)}</div>
                              </div>
                              <div class="bg-surface-container-low p-4 rounded">
                                  <div class="text-xs text-slate-400 uppercase mb-1">P-Value</div>
                                  <div class="text-2xl font-bold" style="color: ${d};">${t.p_value.toFixed(4)}</div>
                                  <div class="text-xs text-slate-500 mt-1">vs Threshold: 0.01</div>
                              </div>
                          </div>

                          <div class="p-4 bg-surface-container-low rounded border border-outline-variant/20 text-sm text-slate-300">
                              <p><strong>Why this matters:</strong> P-value \u2265 0.01 indicates the data passes randomness test. Lower values suggest bias.</p>
                              <p class="mt-2 text-xs text-slate-400">For cryptography, randomness is critical. Biased sequences can be predicted and exploited.</p>
                          </div>
                      </div>`;break}}s+=c+"</div>",Nt.innerHTML=s}}function O(){if(!j)return;let t=j.querySelector(".material-symbols-outlined"),n=j.querySelector("span:last-child");if(F)clearInterval(F),F=null,t&&(t.textContent="play_arrow"),n&&(n.textContent="Play");else{E>=T.length-1&&(E=0);let e=parseInt(tt.value,10);F=setInterval(()=>{Q&&Q.click()},e),t&&(t.textContent="pause"),n&&(n.textContent="Pause")}}Ht&&Ht.addEventListener("click",()=>{dt.value&&ut(!1,dt.value)}),Pt&&Pt.addEventListener("click",()=>{dt.value="",ut(!0)}),At&&At.addEventListener("click",()=>{E>0&&(E--,pt())}),Q&&Q.addEventListener("click",()=>{E<T.length-1?(E++,pt()):F&&O()}),j&&j.addEventListener("click",O),tt&&Rt&&tt.addEventListener("input",()=>{let t=(parseInt(tt.value,10)/1e3).toFixed(1);Rt.textContent=`${t}s / step`,F&&(O(),O())}),setTimeout(()=>ut(!0),500)})});ee();})();
