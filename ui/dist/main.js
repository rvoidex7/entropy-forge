"use strict";(()=>{var Ne=(o,a)=>()=>(o&&(a=o(o=0)),a);var Je=(o,a)=>()=>(a||o((a={exports:{}}).exports,a),a.exports);var w=(o,a,l)=>new Promise((p,c)=>{var K=b=>{try{C(l.next(b))}catch(S){c(S)}},ae=b=>{try{C(l.throw(b))}catch(S){c(S)}},C=b=>b.done?p(b.value):Promise.resolve(b.value).then(K,ae);C((l=l.apply(o,a)).next())});function f(o,a,l,p){if(l==="a"&&!p)throw new TypeError("Private accessor was defined without a getter");if(typeof a=="function"?o!==a||!p:!a.has(o))throw new TypeError("Cannot read private member from an object whose class did not declare it");return l==="m"?p:l==="a"?p.call(o):p?p.value:a.get(o)}function U(o,a,l,p,c){if(p==="m")throw new TypeError("Private method is not writable");if(p==="a"&&!c)throw new TypeError("Private accessor was defined without a setter");if(typeof a=="function"?o!==a||!c:!a.has(o))throw new TypeError("Cannot write private member to an object whose class did not declare it");return p==="a"?c.call(o,l):c?c.value=l:a.set(o,l),l}var We=Ne(()=>{});function et(o,a=!1){return window.__TAURI_INTERNALS__.transformCallback(o,a)}function Oe(p){return w(this,arguments,function*(o,a={},l){return window.__TAURI_INTERNALS__.invoke(o,a,l)})}var N,v,V,ue,Qe,Ke,je,ze=Ne(()=>{We();Ke="__TAURI_TO_IPC_KEY__";je=class{constructor(a){N.set(this,void 0),v.set(this,0),V.set(this,[]),ue.set(this,void 0),U(this,N,a||(()=>{}),"f"),this.id=et(l=>{let p=l.index;if("end"in l){p==f(this,v,"f")?this.cleanupCallback():U(this,ue,p,"f");return}let c=l.message;if(p==f(this,v,"f")){for(f(this,N,"f").call(this,c),U(this,v,f(this,v,"f")+1,"f");f(this,v,"f")in f(this,V,"f");){let K=f(this,V,"f")[f(this,v,"f")];f(this,N,"f").call(this,K),delete f(this,V,"f")[f(this,v,"f")],U(this,v,f(this,v,"f")+1,"f")}f(this,v,"f")===f(this,ue,"f")&&this.cleanupCallback()}else f(this,V,"f")[p]=c})}cleanupCallback(){window.__TAURI_INTERNALS__.unregisterCallback(this.id)}set onmessage(a){U(this,N,a,"f")}get onmessage(){return f(this,N,"f")}[(N=new WeakMap,v=new WeakMap,V=new WeakMap,ue=new WeakMap,Ke)](){return`__CHANNEL__:${this.id}`}toJSON(){return this[Ke]()}};Qe=new WeakMap});var tt=Je(re=>{ze();var W=(l,...p)=>w(re,[l,...p],function*(o,a={}){if(window.__TAURI_INTERNALS__)return yield Oe(o,a);throw console.warn(`[Tauri Mock] Called invoke('${o}') with args:`,a),new Error("Tauri API is not available in the browser.")});document.addEventListener("DOMContentLoaded",()=>{let o=document.querySelectorAll(".tab-link, .mobile-tab-link"),a=document.querySelectorAll(".tab-panel");function l(e){a.forEach(t=>{t.classList.remove("active")});let n=document.getElementById(e);n&&n.classList.add("active"),document.querySelectorAll(".tab-link").forEach(t=>{if(t.getAttribute("data-target")===e){t.classList.remove("text-slate-500"),t.classList.add("text-[#00FF88]","bg-[#00FF88]/10","border-l-2","border-[#00FF88]");let s=t.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 1")}else{t.classList.add("text-slate-500"),t.classList.remove("text-[#00FF88]","bg-[#00FF88]/10","border-l-2","border-[#00FF88]");let s=t.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 0")}}),document.querySelectorAll(".mobile-tab-link").forEach(t=>{if(t.getAttribute("data-target")===e){t.classList.remove("text-gray-500"),t.classList.add("text-[#00FF88]","border-t-2","border-[#00FF88]");let s=t.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 1")}else{t.classList.add("text-gray-500"),t.classList.remove("text-[#00FF88]","border-t-2","border-[#00FF88]");let s=t.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 0")}})}o.forEach(e=>{e.addEventListener("click",n=>{n.preventDefault();let t=n.currentTarget.getAttribute("data-target");t&&l(t)})}),document.querySelectorAll(".sub-tab-link").forEach(e=>{e.addEventListener("click",n=>{n.preventDefault(),l("tab-learn");let t=n.currentTarget.getAttribute("data-target");if(t){let s=document.getElementById(t);s&&setTimeout(()=>{s.scrollIntoView({behavior:"smooth",block:"start"})},100)}})});let c=document.getElementById("use-input"),K=document.getElementById("use-input-length"),ae=document.getElementById("use-hex-check"),C=document.getElementById("use-output"),b=document.getElementById("use-encrypt-btn"),S=document.getElementById("use-keystream-grid");c&&K&&c.addEventListener("input",()=>{K.textContent=`LENGTH: ${c.value.length} BYTES`}),b&&c&&C&&ae&&S&&b.addEventListener("click",()=>w(re,null,function*(){let e=c.value;if(e)try{let n=yield W("encrypt_decrypt",{plaintext:e,hexOutput:ae.checked});C.value=n.ciphertext,S.innerHTML="";let t=n.keystream_bytes||[],s=e.length,r=S.parentElement;if(r){let i=r.clientWidth,d=20,u=4,y=i-16,g=d+u,ve=Math.floor(y/g),he=Math.max(1,ve);S.style.gridTemplateColumns=`repeat(${he}, 20px)`}for(let i=0;i<s;i++){let d=document.createElement("div"),u=t[i]||0,m=Math.round(u/255*255),y=`rgb(${m}, ${m}, ${m})`;d.style.backgroundColor=y,d.title=`Byte ${i}: 0x${u.toString(16).toUpperCase().padStart(2,"0")} (${u})`,S.appendChild(d)}}catch(n){console.error(n),C.value=`Error: ${n}`}}));let Ee=document.getElementById("use-help-link");Ee&&Ee.addEventListener("click",()=>{l("tab-learn");let e=document.getElementById("learn-input");e&&c&&c.value&&(e.value=c.value,e.dispatchEvent(new Event("input")))});let G=document.getElementById("test-size-slider"),Ie=document.getElementById("test-size-label"),Le=document.getElementById("test-run-btn"),_e=document.getElementById("test-results-section");G&&Ie&&G.addEventListener("input",()=>{let e=parseFloat(G.value),n=Math.round(Math.pow(10,e));Ie.textContent=`${n.toLocaleString()} BYTES`}),Le&&G&&_e&&Le.addEventListener("click",()=>w(re,null,function*(){let e=parseFloat(G.value),n=Math.round(Math.pow(10,e));_e.classList.remove("hidden");let t=document.getElementById("test-nist-tbody");t&&(t.innerHTML='<tr><td colspan="3" class="text-center py-4">Running tests...</td></tr>');try{let s=yield W("run_quality_tests",{sampleSize:n});document.getElementById("test-shannon-val").textContent=s.shannon_entropy.toFixed(4),document.getElementById("test-shannon-bar").style.width=`${s.shannon_entropy/8*100}%`,document.getElementById("test-min-val").textContent=s.min_entropy.toFixed(4),document.getElementById("test-min-bar").style.width=`${s.min_entropy/8*100}%`,document.getElementById("test-mean-val").textContent=s.mean.toFixed(3),document.getElementById("test-chi-val").textContent=s.chi_square.toFixed(2),document.getElementById("test-longest-run-val").textContent=s.longest_run.toString(),document.getElementById("test-score-val").textContent=s.overall_score.toFixed(1),t&&(t.innerHTML="",s.nist_results.forEach(r=>{let i=document.createElement("tr");i.className="border-b border-outline-variant/10 hover:bg-surface-container-low transition-colors group";let d=r.passed?'<span class="bg-[#00FF88]/10 text-[#00FF88] border border-[#00FF88]/30 px-3 py-1 font-bold">\u2713 PASS</span>':'<span class="bg-error-container/20 text-error border border-error/30 px-3 py-1 font-bold">\u2717 FAIL</span>';i.innerHTML=`
                            <td class="px-6 py-4 font-bold uppercase tracking-tight">${r.name}</td>
                            <td class="px-6 py-4 font-mono">${r.p_value.toFixed(6)}</td>
                            <td class="px-6 py-4 text-right">${d}</td>
                        `,t.appendChild(i)}))}catch(s){console.error(s),t&&(t.innerHTML=`<tr><td colspan="3" class="text-center text-error py-4">Error: ${s}</td></tr>`)}}));let j=document.getElementById("bench-size-slider"),oe=document.getElementById("bench-size-label"),we=document.getElementById("bench-run-btn"),Se=document.getElementById("bench-results-section");j&&oe&&(j.addEventListener("input",()=>{let e=parseFloat(j.value),n=Math.round(Math.pow(10,e));n>=1e6?oe.textContent=`${(n/1e6).toFixed(1)}M bytes`:n>=1e3?oe.textContent=`${(n/1e3).toFixed(0)}K bytes`:oe.textContent=`${n} bytes`}),j.dispatchEvent(new Event("input"))),we&&j&&Se&&we.addEventListener("click",()=>w(re,null,function*(){let e=parseFloat(j.value),n=Math.round(Math.pow(10,e));Se.classList.remove("hidden");try{let t=yield W("run_benchmark",{bytes:n});document.getElementById("bench-throughput").textContent=t.throughput_mbps.toFixed(2),document.getElementById("bench-latency").textContent=t.latency_us.toFixed(2);let s=t.bytes_generated.toLocaleString();document.getElementById("bench-details").textContent=`Generated ${s} bytes in ${t.duration_secs.toFixed(3)} seconds`}catch(t){console.error(t),document.getElementById("bench-details").textContent=`Error: ${t}`}}));let ke=document.querySelectorAll(".learn-btn"),Ue=document.querySelectorAll(".learn-section");ke.forEach(e=>{e.addEventListener("click",n=>{let t=n.currentTarget.getAttribute("data-learn");ke.forEach(s=>{s.classList.remove("active","bg-primary-container","text-on-primary-container"),s.classList.add("bg-surface-container","text-slate-400")}),n.currentTarget.classList.remove("bg-surface-container","text-slate-400"),n.currentTarget.classList.add("active","bg-primary-container","text-on-primary-container"),Ue.forEach(s=>{s.id===`learn-${t}`?s.classList.add("active"):s.classList.remove("active")})})});let h=[],x=0,B=null,M=document.getElementById("learn-input"),X=document.getElementById("learn-prev"),k=document.getElementById("learn-next"),H=document.getElementById("learn-play-pause"),ie=document.getElementById("learn-speed-slider"),Te=document.getElementById("learn-speed-text"),Fe=document.getElementById("learn-xor-step-info"),le=document.getElementById("learn-xor-content"),$e=document.getElementById("learn-progress-text"),Ce=document.getElementById("learn-progress-slider");function Be(){return w(this,null,function*(){if(M.value)try{h=(yield W("get_xor_steps",{text:M.value})).steps,x=0,B&&Z(),pe()}catch(e){console.error(e)}})}function pe(){if(h.length===0){le&&(le.innerHTML="No steps.");return}let e=h[x];Fe&&(Fe.textContent=`STEP_IDX: ${(x+1).toString().padStart(3,"0")} / ${h.length.toString().padStart(3,"0")}`);let n=Math.round((x+1)/h.length*100);if($e&&($e.textContent=`${n}%`),Ce&&(Ce.value=n.toString()),le){let t=e.input_binary.split("").map(i=>`<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-cyan-400">${i}</span>`).join(""),s=e.keystream_binary.split("").map(i=>`<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-orange-400">${i}</span>`).join(""),r=e.result_binary.split("").map(i=>`<span class="w-6 h-8 bg-primary-container/10 border border-green-400/30 flex items-center justify-center font-black text-green-400">${i}</span>`).join("");le.innerHTML=`
             <div class="space-y-6">
                 <h3 class="text-lg font-bold text-primary">Step ${x+1} of ${h.length}: Encrypting '${e.character}'</h3>
                 
                 <div class="bg-surface-container-low p-4 relative overflow-hidden border-l-2 border-[#00FF88] rounded">
                     <div class="flex justify-between items-start mb-4">
                         <div>
                             <span class="text-[10px] text-secondary uppercase font-bold">Input Character</span>
                             <div class="text-4xl font-black text-primary">'${e.character}'</div>
                         </div>
                         <div class="text-right">
                             <span class="text-[10px] text-outline-variant uppercase">ASCII Decimal</span>
                             <div class="text-xl font-bold">${e.input_byte}</div>
                         </div>
                     </div>

                     <div class="space-y-4 pt-4 border-t border-outline-variant/30">
                         <div class="flex items-center justify-between">
                             <span class="text-[10px] text-cyan-400 text-left uppercase w-20 font-bold">Plaintext</span>
                             <div class="flex gap-1">${t}</div>
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
                             <div class="flex gap-1">${r}</div>
                         </div>
                     </div>
                 </div>
             </div>
             `}Ve()}function Ve(){let e=document.getElementById("learn-xor-progress");if(!e)return;if(h.length===0){e.classList.add("hidden");return}e.classList.remove("hidden");let n=e.querySelector(".flex.flex-wrap.gap-2");if(!n)return;let t="";h.forEach((s,r)=>{let i=s.result_byte.toString(16).toUpperCase().padStart(2,"0"),d="px-3 py-2 rounded font-mono text-sm font-bold transition-colors ";r===x?d+="bg-green-500/30 border border-green-400 text-green-300":r<x?d+="bg-surface-container-highest text-slate-400 border border-outline-variant/20":d+="bg-surface-container text-slate-500 border border-outline-variant/20",t+=`<span class="${d}">${i}</span>`}),n.innerHTML=t}M&&(M.addEventListener("input",()=>{Be()}),setTimeout(Be,500)),X&&X.addEventListener("click",()=>{x>0&&(x--,pe())}),k&&k.addEventListener("click",()=>{x<h.length-1?(x++,pe()):B&&Z()});function Z(){if(!H)return;let e=H.querySelector(".material-symbols-outlined");if(B)clearInterval(B),B=null,e&&(e.textContent="play_arrow");else{x>=h.length-1&&(x=0);let n=parseInt(ie.value,10);B=setInterval(()=>{k&&k.click()},n),e&&(e.textContent="pause")}}H&&H.addEventListener("click",Z),ie&&Te&&ie.addEventListener("input",()=>{let e=(parseInt(ie.value,10)/1e3).toFixed(1);Te.textContent=`${e}s / step`,B&&(Z(),Z())});let E=document.getElementById("learn-xor-collapsible"),me=document.getElementById("learn-xor-details");E&&me&&E.addEventListener("click",()=>{me.classList.toggle("hidden");let e=!me.classList.contains("hidden");E.setAttribute("aria-expanded",e?"true":"false");let n=E.querySelector(".material-symbols-outlined");n&&(n.textContent=e?"expand_less":"expand_more")});let I=document.getElementById("learn-entropy-collapsible"),fe=document.getElementById("learn-entropy-details");I&&fe&&I.addEventListener("click",()=>{fe.classList.toggle("hidden");let e=!fe.classList.contains("hidden");I.setAttribute("aria-expanded",e?"true":"false");let n=I.querySelector(".material-symbols-outlined");n&&(n.textContent=e?"expand_less":"expand_more")});let L=document.getElementById("learn-nist-collapsible"),ye=document.getElementById("learn-nist-details");L&&ye&&L.addEventListener("click",()=>{ye.classList.toggle("hidden");let e=!ye.classList.contains("hidden");L.setAttribute("aria-expanded",e?"true":"false");let n=L.querySelector(".material-symbols-outlined");n&&(n.textContent=e?"expand_less":"expand_more")});let O=[],T=0,A=null,Y=document.getElementById("learn-entropy-input"),J=document.getElementById("learn-entropy-prev"),q=document.getElementById("learn-entropy-next"),F=document.getElementById("learn-entropy-play-pause"),Me=document.getElementById("learn-entropy-step-info"),He=document.getElementById("learn-entropy-content"),Q=document.getElementById("learn-entropy-calculate"),ce=document.getElementById("learn-entropy-speed-slider"),Ae=document.getElementById("learn-entropy-speed-text");function Ge(){return w(this,null,function*(){if(Y.value)try{O=(yield W("get_entropy_steps",{text:Y.value})).steps,T=0,A&&ee(),be()}catch(e){console.error(e)}})}function be(){var t,s;if(O.length===0)return;let e=O[T],n=e.step_type;if(Me&&(Me.textContent=`Step ${T+1} / ${O.length}`),He){let r=`<table class="w-full text-sm">
                 <thead>
                     <tr class="border-b border-outline-variant/30 text-left">
                         <th class="p-2 font-bold text-primary">Byte</th>
                         <th class="p-2 font-bold text-primary">Count</th>`;n!=="CountBytes"&&(r+='<th class="p-2 font-bold text-primary">Probability</th>'),(n==="CalculateContributions"||n==="SumEntropy"||n==="Interpret")&&(r+='<th class="p-2 font-bold text-primary">Contribution</th>'),r+=`<th class="p-2 font-bold text-primary">Visual</th>
                     </tr>
                 </thead>
                 <tbody>`;let i=Object.keys(e.byte_counts).sort(),d=Math.max(...Object.values(e.byte_counts),1);for(let m of i){let y=parseInt(m),g=e.byte_counts[m],ve=((t=e.probabilities)==null?void 0:t[m])||0,he=((s=e.entropy_contributions)==null?void 0:s[m])||0,Xe=y>=32&&y<=126?String.fromCharCode(y):`0x${y.toString(16).toUpperCase().padStart(2,"0")}`,Ze=g/d*100,Ye=g===1?"#00FF88":`rgb(255, ${100+(155-Math.min(g*20,155))}, 100)`;r+=`<tr class="border-b border-outline-variant/20 hover:bg-surface-container-low transition-colors">
                     <td class="p-2 font-mono text-slate-300">'${Xe}'</td>
                     <td class="p-2 font-mono text-slate-300">${g}</td>`,n!=="CountBytes"&&(r+=`<td class="p-2 font-mono text-slate-300">${ve.toFixed(3)}</td>`),(n==="CalculateContributions"||n==="SumEntropy"||n==="Interpret")&&(r+=`<td class="p-2 font-mono text-slate-300">${he.toFixed(3)} bits</td>`),r+=`<td class="p-2"><div class="h-6 bg-surface-container rounded" style="width: ${Ze}%; background-color: ${Ye};"></div></td>
                     </tr>`}r+="</tbody></table>";let u="";switch(n){case"CountBytes":u=`<div class="mt-6 p-4 bg-surface-container-low rounded border-l-2 border-primary">
                         <p class="text-sm text-slate-300">
                             <strong>\u{1F4A1} What's happening:</strong> We count how many times each unique byte appears in the input. This frequency distribution is the foundation for entropy calculation.
                         </p>
                     </div>`;break;case"CalculateProbabilities":u=`<div class="mt-6 space-y-3">
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
                     </div>`;break;case"CalculateContributions":u=`<div class="mt-6 space-y-3">
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
                     </div>`;break;case"SumEntropy":case"Interpret":{let m=e.max_entropy>0?e.total_entropy/e.max_entropy*100:0,y="Moderate entropy.",g="\u26A0\uFE0F";e.current_entropy_sum<2?(y="Low entropy. The input is very predictable.",g="\u{1F4C9}"):m>80&&(y="High entropy! The input looks quite random or uses many different characters.",g="\u{1F525}"),u=`<div class="mt-6 space-y-4">
                          <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                              <div class="bg-surface-container-low p-4 rounded border border-primary/30">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Total Entropy</div>
                                  <div class="text-2xl font-bold text-primary">${e.current_entropy_sum.toFixed(4)}</div>
                                  <div class="text-xs text-slate-500 mt-1">bits/byte</div>
                              </div>
                              <div class="bg-surface-container-low p-4 rounded border border-primary/30">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Maximum Possible</div>
                                  <div class="text-2xl font-bold text-primary">${e.max_entropy.toFixed(4)}</div>
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
                                  <strong>${g} Interpretation:</strong>
                              </p>
                              <p class="text-sm text-slate-300">
                                  ${y}
                              </p>
                              ${e.current_entropy_sum<2?'<p class="text-sm text-slate-400 mt-2 italic">For cryptography, you generally want entropy > 7 bits/byte for strong randomness.</p>':""}
                          </div>
                      </div>`;break}}He.innerHTML=r+u}}function ee(){if(!F)return;let e=F.querySelector(".material-symbols-outlined"),n=F.querySelector("span:last-child");if(A)clearInterval(A),A=null,e&&(e.textContent="play_arrow"),n&&(n.textContent="Play");else{T>=O.length-1&&(T=0);let t=parseInt(ce.value,10);A=setInterval(()=>{q&&q.click()},t),e&&(e.textContent="pause"),n&&(n.textContent="Pause")}}Q&&Q.addEventListener("click",Ge),J&&J.addEventListener("click",()=>{T>0&&(T--,be())}),q&&q.addEventListener("click",()=>{T<O.length-1?(T++,be()):A&&ee()}),F&&F.addEventListener("click",ee),ce&&Ae&&ce.addEventListener("input",()=>{let e=(parseInt(ce.value,10)/1e3).toFixed(1);Ae.textContent=`${e}s / step`,A&&(ee(),ee())});let P=[],_=0,R=null,z=document.getElementById("learn-nist-input"),te=document.getElementById("learn-nist-analyze"),qe=document.getElementById("learn-nist-random"),ne=document.getElementById("learn-nist-prev"),D=document.getElementById("learn-nist-next"),$=document.getElementById("learn-nist-play-pause"),Pe=document.getElementById("learn-nist-step-info"),Re=document.getElementById("learn-nist-content"),de=document.getElementById("learn-nist-speed-slider"),De=document.getElementById("learn-nist-speed-text");function xe(e=!0,n=""){return w(this,null,function*(){try{let t;e?t=yield W("get_nist_steps_random",{count:64}):t=yield W("get_nist_steps",{text:n}),P=t.steps,_=0,R&&se(),ge()}catch(t){console.error(t)}})}function ge(){if(P.length===0)return;let e=P[_],n=e.step_type;if(Pe&&(Pe.textContent=`Step ${_+1} / ${P.length}`),Re){let t="";for(let u=0;u<e.bits.length;u++){u>0&&u%8===0&&(t+='<span class="mx-2"></span>');let m=e.bits[u];t+=`<span class="inline-block w-4 text-center font-bold ${m===1?"text-[#00FF88]":"text-slate-400"}">${m}</span>`}let s=`<div class="space-y-6">
                 <h3 class="text-lg font-bold text-[#A855F7]">Step ${_+1} of ${P.length}: ${n.replace(/([A-Z])/g," $1").trim()}</h3>

                 <div>
                     <h4 class="text-sm font-bold text-slate-300 mb-2">Bit Sequence (${e.bits.length} bits):</h4>
                     <div class="bg-surface-container-lowest p-4 rounded border border-outline-variant/20 font-mono text-sm break-all">
                         ${t}
                     </div>
                 </div>`,r=e.bits.length,i=r>0?e.ones_count/r*100:0;n!=="ConvertToBits"&&(s+=`<div class="grid grid-cols-2 gap-4">
                     <div class="bg-surface-container-low p-4 rounded border border-outline-variant/20">
                         <div class="text-xs text-slate-400 uppercase font-bold mb-1">Ones Count</div>
                         <div class="text-3xl font-bold text-[#00FF88]">${e.ones_count}</div>
                     </div>
                     <div class="bg-surface-container-low p-4 rounded border border-outline-variant/20">
                         <div class="text-xs text-slate-400 uppercase font-bold mb-1">Zeros Count</div>
                         <div class="text-3xl font-bold text-slate-400">${e.zeros_count}</div>
                     </div>
                 </div>

                 <!-- Balance Visualization -->
                 <div>
                     <h4 class="text-sm font-bold text-slate-300 mb-2">Distribution Balance (Ideal: 50% Ones)</h4>
                     <div class="relative h-12 bg-slate-600 rounded overflow-hidden">
                         <div class="absolute h-full bg-[#00FF88] transition-all" style="width: ${i}%;"></div>
                         <div class="absolute top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2 w-0.5 h-full bg-black"></div>
                         <div class="absolute inset-0 flex items-center justify-center font-bold text-sm text-white mix-blend-multiply">
                             ${i.toFixed(1)}% Ones (${e.ones_count}/${r} bits)
                         </div>
                     </div>
                 </div>`);let d="";switch(n){case"ConvertToBits":d=`<div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                         <p class="text-sm text-slate-300">
                             <strong>\u{1F4A1} What's happening:</strong> Each byte becomes 8 binary digits. White = 0, Green = 1.
                         </p>
                     </div>`;break;case"CountOnesZeros":d=`<div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                         <p class="text-sm text-slate-300 mb-2">
                             <strong>\u{1F4A1} What's happening:</strong> If the data is truly random, we expect roughly 50% ones and 50% zeros, like fair coin flips.
                         </p>
                         <p class="text-sm text-slate-400 italic">
                             Current ratio: ${i.toFixed(1)}% ones vs ${(100-i).toFixed(1)}% zeros
                         </p>
                     </div>`;break;case"CalculateStatistic":d=`<div class="space-y-3">
                         <div class="bg-surface-container-low p-4 rounded border-l-2 border-[#A855F7]">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>\u{1F4A1} Formula:</strong> S_obs = |Sum| \xF7 \u221A(n)
                             </p>
                             <p class="text-sm text-slate-400">Where Sum = (count_ones \xD7 +1) + (count_zeros \xD7 -1)</p>
                             <p class="text-sm text-slate-400">Result: ${e.sum} \xF7 \u221A${r} = ${e.s_obs.toFixed(4)}</p>
                         </div>
                         <div class="p-3 bg-surface-container rounded text-xs text-slate-400">
                             <p>S_obs tells us how far from balanced (0) the sequence is. Closer to 0 = more balanced and random-looking.</p>
                         </div>
                     </div>`;break;case"CalculatePValue":d=`<div class="space-y-3">
                         <div class="bg-surface-container-low p-4 rounded border-l-2 border-[#A855F7]">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>\u{1F4A1} Formula:</strong> P-value = erfc(S_obs \xF7 \u221A2)
                             </p>
                             <p class="text-sm text-slate-400">Result: erfc(${e.s_obs.toFixed(4)} \xF7 1.414) = ${e.p_value.toFixed(4)}</p>
                         </div>
                         <div class="p-3 bg-surface-container rounded text-xs text-slate-400">
                             <p><strong>Interpretation:</strong> P-value = ${(e.p_value*100).toFixed(1)}% probability that a truly random sequence would show this much deviation.</p>
                         </div>
                     </div>`;break;case"Interpret":{let u=e.passed?"#00FF88":"#ff6b6b",m=e.passed?"\u2705":"\u274C",y=e.passed?"PASS":"FAIL",g=e.passed?"The sequence looks RANDOM!":"The sequence is BIASED toward 1s or 0s.";d=`<div class="space-y-4">
                          <div class="p-4 rounded border-l-4" style="border-color: ${u}; background: rgba(0,0,0,0.2);">
                              <div class="flex items-center gap-2 mb-2">
                                  <span style="color: ${u}" class="text-2xl font-bold">${m} ${y}</span>
                              </div>
                              <p class="text-lg text-slate-300 font-bold mb-2">${g}</p>
                          </div>

                          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                              <div class="bg-surface-container-low p-4 rounded">
                                  <div class="text-xs text-slate-400 uppercase mb-1">Test Statistic (S_obs)</div>
                                  <div class="text-2xl font-bold text-slate-300">${e.s_obs.toFixed(4)}</div>
                              </div>
                              <div class="bg-surface-container-low p-4 rounded">
                                  <div class="text-xs text-slate-400 uppercase mb-1">P-Value</div>
                                  <div class="text-2xl font-bold" style="color: ${u};">${e.p_value.toFixed(4)}</div>
                                  <div class="text-xs text-slate-500 mt-1">vs Threshold: 0.01</div>
                              </div>
                          </div>

                          <div class="p-4 bg-surface-container-low rounded border border-outline-variant/20 text-sm text-slate-300">
                              <p><strong>Why this matters:</strong> P-value \u2265 0.01 indicates the data passes randomness test. Lower values suggest bias.</p>
                              <p class="mt-2 text-xs text-slate-400">For cryptography, randomness is critical. Biased sequences can be predicted and exploited.</p>
                          </div>
                      </div>`;break}}s+=d+"</div>",Re.innerHTML=s}}function se(){if(!$)return;let e=$.querySelector(".material-symbols-outlined"),n=$.querySelector("span:last-child");if(R)clearInterval(R),R=null,e&&(e.textContent="play_arrow"),n&&(n.textContent="Play");else{_>=P.length-1&&(_=0);let t=parseInt(de.value,10);R=setInterval(()=>{D&&D.click()},t),e&&(e.textContent="pause"),n&&(n.textContent="Pause")}}te&&te.addEventListener("click",()=>{z.value&&xe(!1,z.value)}),qe&&qe.addEventListener("click",()=>{z.value="",xe(!0)}),ne&&ne.addEventListener("click",()=>{_>0&&(_--,ge())}),D&&D.addEventListener("click",()=>{_<P.length-1?(_++,ge()):R&&se()}),$&&$.addEventListener("click",se),de&&De&&de.addEventListener("input",()=>{let e=(parseInt(de.value,10)/1e3).toFixed(1);De.textContent=`${e}s / step`,R&&(se(),se())}),setTimeout(()=>xe(!0),500),c==null||c.addEventListener("keydown",e=>{e.key==="Enter"&&!e.shiftKey&&(e.preventDefault(),b==null||b.click())}),M==null||M.addEventListener("keydown",e=>{e.key==="Enter"&&(e.preventDefault(),k&&k.click())}),Y==null||Y.addEventListener("keydown",e=>{e.key==="Enter"&&(e.preventDefault(),Q==null||Q.click())}),z==null||z.addEventListener("keydown",e=>{e.key==="Enter"&&(e.preventDefault(),te==null||te.click())}),document.addEventListener("keydown",e=>{if(e.key==="Escape"){let n=document.getElementById("learn-xor-details"),t=document.getElementById("learn-entropy-details"),s=document.getElementById("learn-nist-details");if(n&&!n.classList.contains("hidden")){n.classList.add("hidden"),E==null||E.setAttribute("aria-expanded","false");let r=E==null?void 0:E.querySelector(".material-symbols-outlined");r&&(r.textContent="expand_more")}if(t&&!t.classList.contains("hidden")){t.classList.add("hidden"),I==null||I.setAttribute("aria-expanded","false");let r=I==null?void 0:I.querySelector(".material-symbols-outlined");r&&(r.textContent="expand_more")}if(s&&!s.classList.contains("hidden")){s.classList.add("hidden"),L==null||L.setAttribute("aria-expanded","false");let r=L==null?void 0:L.querySelector(".material-symbols-outlined");r&&(r.textContent="expand_more")}}}),document.addEventListener("keydown",e=>{let n=document.querySelector(".tab-panel.active");if(!(!n||n.id!=="tab-learn")){if(e.key==="ArrowRight"&&(e.ctrlKey||e.metaKey)){e.preventDefault();let t=n.querySelector(".learn-section.active");(t==null?void 0:t.id)==="learn-xor"?k==null||k.click():(t==null?void 0:t.id)==="learn-entropy"?q==null||q.click():(t==null?void 0:t.id)==="learn-nist"&&(D==null||D.click())}if(e.key==="ArrowLeft"&&(e.ctrlKey||e.metaKey)){e.preventDefault();let t=n.querySelector(".learn-section.active");(t==null?void 0:t.id)==="learn-xor"?X==null||X.click():(t==null?void 0:t.id)==="learn-entropy"?J==null||J.click():(t==null?void 0:t.id)==="learn-nist"&&(ne==null||ne.click())}if(e.key===" "&&e.ctrlKey){e.preventDefault();let t=n.querySelector(".learn-section.active");(t==null?void 0:t.id)==="learn-xor"?H==null||H.click():(t==null?void 0:t.id)==="learn-entropy"?F==null||F.click():(t==null?void 0:t.id)==="learn-nist"&&($==null||$.click())}}}),document.addEventListener("keydown",e=>{if(e.ctrlKey&&!e.shiftKey&&!e.altKey&&!e.metaKey){let n=parseInt(e.key,10);if(n>=1&&n<=4){e.preventDefault();let s={1:"tab-use",2:"tab-test",3:"tab-bench",4:"tab-learn"}[n];s&&l(s)}}})})});tt();})();
