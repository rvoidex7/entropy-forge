"use strict";(()=>{var Ne=(i,a)=>()=>(i&&(a=i(i=0)),a);var Je=(i,a)=>()=>(a||i((a={exports:{}}).exports,a),a.exports);var w=(i,a,c)=>new Promise((p,d)=>{var K=b=>{try{C(c.next(b))}catch(S){d(S)}},ae=b=>{try{C(c.throw(b))}catch(S){d(S)}},C=b=>b.done?p(b.value):Promise.resolve(b.value).then(K,ae);C((c=c.apply(i,a)).next())});function f(i,a,c,p){if(c==="a"&&!p)throw new TypeError("Private accessor was defined without a getter");if(typeof a=="function"?i!==a||!p:!a.has(i))throw new TypeError("Cannot read private member from an object whose class did not declare it");return c==="m"?p:c==="a"?p.call(i):p?p.value:a.get(i)}function U(i,a,c,p,d){if(p==="m")throw new TypeError("Private method is not writable");if(p==="a"&&!d)throw new TypeError("Private accessor was defined without a setter");if(typeof a=="function"?i!==a||!d:!a.has(i))throw new TypeError("Cannot write private member to an object whose class did not declare it");return p==="a"?d.call(i,c):d?d.value=c:a.set(i,c),c}var We=Ne(()=>{});function et(i,a=!1){return window.__TAURI_INTERNALS__.transformCallback(i,a)}function Oe(p){return w(this,arguments,function*(i,a={},c){return window.__TAURI_INTERNALS__.invoke(i,a,c)})}var N,v,V,ue,Qe,Ke,je,ze=Ne(()=>{We();Ke="__TAURI_TO_IPC_KEY__";je=class{constructor(a){N.set(this,void 0),v.set(this,0),V.set(this,[]),ue.set(this,void 0),U(this,N,a||(()=>{}),"f"),this.id=et(c=>{let p=c.index;if("end"in c){p==f(this,v,"f")?this.cleanupCallback():U(this,ue,p,"f");return}let d=c.message;if(p==f(this,v,"f")){for(f(this,N,"f").call(this,d),U(this,v,f(this,v,"f")+1,"f");f(this,v,"f")in f(this,V,"f");){let K=f(this,V,"f")[f(this,v,"f")];f(this,N,"f").call(this,K),delete f(this,V,"f")[f(this,v,"f")],U(this,v,f(this,v,"f")+1,"f")}f(this,v,"f")===f(this,ue,"f")&&this.cleanupCallback()}else f(this,V,"f")[p]=d})}cleanupCallback(){window.__TAURI_INTERNALS__.unregisterCallback(this.id)}set onmessage(a){U(this,N,a,"f")}get onmessage(){return f(this,N,"f")}[(N=new WeakMap,v=new WeakMap,V=new WeakMap,ue=new WeakMap,Ke)](){return`__CHANNEL__:${this.id}`}toJSON(){return this[Ke]()}};Qe=new WeakMap});var tt=Je(re=>{ze();var W=(c,...p)=>w(re,[c,...p],function*(i,a={}){if(window.__TAURI_INTERNALS__)return yield Oe(i,a);throw console.warn(`[Tauri Mock] Called invoke('${i}') with args:`,a),new Error("Tauri API is not available in the browser.")});document.addEventListener("DOMContentLoaded",()=>{let i=document.querySelectorAll(".tab-link, .mobile-tab-link"),a=document.querySelectorAll(".tab-panel");function c(e){a.forEach(t=>{t.classList.remove("active")});let n=document.getElementById(e);n&&n.classList.add("active"),document.querySelectorAll(".tab-link").forEach(t=>{if(t.getAttribute("data-target")===e){t.classList.remove("text-slate-500"),t.classList.add("text-[#00FF88]","bg-[#00FF88]/10","border-l-2","border-[#00FF88]");let s=t.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 1")}else{t.classList.add("text-slate-500"),t.classList.remove("text-[#00FF88]","bg-[#00FF88]/10","border-l-2","border-[#00FF88]");let s=t.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 0")}}),document.querySelectorAll(".mobile-tab-link").forEach(t=>{if(t.getAttribute("data-target")===e){t.classList.remove("text-gray-500"),t.classList.add("text-[#00FF88]","border-t-2","border-[#00FF88]");let s=t.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 1")}else{t.classList.add("text-gray-500"),t.classList.remove("text-[#00FF88]","border-t-2","border-[#00FF88]");let s=t.querySelector(".material-symbols-outlined");s&&(s.style.fontVariationSettings="'FILL' 0")}})}i.forEach(e=>{e.addEventListener("click",n=>{n.preventDefault();let t=n.currentTarget.getAttribute("data-target");t&&c(t)})}),document.querySelectorAll(".sub-tab-link").forEach(e=>{e.addEventListener("click",n=>{n.preventDefault(),c("tab-learn");let t=n.currentTarget.getAttribute("data-target");if(t){let s=t.replace("learn-","");document.querySelectorAll(".learn-section").forEach(o=>{o.id===t?o.classList.add("active"):o.classList.remove("active")}),document.querySelectorAll(".learn-btn").forEach(o=>{o.getAttribute("data-learn")===s?(o.classList.remove("bg-surface-container","text-slate-400"),o.classList.add("active","bg-primary-container","text-on-primary-container")):(o.classList.remove("active","bg-primary-container","text-on-primary-container"),o.classList.add("bg-surface-container","text-slate-400"))})}})});let d=document.getElementById("use-input"),K=document.getElementById("use-input-length"),ae=document.getElementById("use-hex-check"),C=document.getElementById("use-output"),b=document.getElementById("use-encrypt-btn"),S=document.getElementById("use-keystream-grid");d&&K&&d.addEventListener("input",()=>{K.textContent=`LENGTH: ${d.value.length} BYTES`}),b&&d&&C&&ae&&S&&b.addEventListener("click",()=>w(re,null,function*(){let e=d.value;if(e)try{let n=yield W("encrypt_decrypt",{plaintext:e,hexOutput:ae.checked});C.value=n.ciphertext,S.innerHTML="";let t=n.keystream_bytes||[],s=e.length,r=S.parentElement;if(r){let l=r.clientWidth,o=20,u=4,y=l-16,g=o+u,ve=Math.floor(y/g),he=Math.max(1,ve);S.style.gridTemplateColumns=`repeat(${he}, 20px)`}for(let l=0;l<s;l++){let o=document.createElement("div"),u=t[l]||0,m=Math.round(u/255*255),y=`rgb(${m}, ${m}, ${m})`;o.style.backgroundColor=y,o.title=`Byte ${l}: 0x${u.toString(16).toUpperCase().padStart(2,"0")} (${u})`,S.appendChild(o)}}catch(n){console.error(n),C.value=`Error: ${n}`}}));let Ee=document.getElementById("use-help-link");Ee&&Ee.addEventListener("click",()=>{c("tab-learn");let e=document.getElementById("learn-input");e&&d&&d.value&&(e.value=d.value,e.dispatchEvent(new Event("input")))});let G=document.getElementById("test-size-slider"),Le=document.getElementById("test-size-label"),_e=document.getElementById("test-run-btn"),Ie=document.getElementById("test-results-section");G&&Le&&G.addEventListener("input",()=>{let e=parseFloat(G.value),n=Math.round(Math.pow(10,e));Le.textContent=`${n.toLocaleString()} BYTES`}),_e&&G&&Ie&&_e.addEventListener("click",()=>w(re,null,function*(){let e=parseFloat(G.value),n=Math.round(Math.pow(10,e));Ie.classList.remove("hidden");let t=document.getElementById("test-nist-tbody");t&&(t.innerHTML='<tr><td colspan="3" class="text-center py-4">Running tests...</td></tr>');try{let s=yield W("run_quality_tests",{sampleSize:n});document.getElementById("test-shannon-val").textContent=s.shannon_entropy.toFixed(4),document.getElementById("test-shannon-bar").style.width=`${s.shannon_entropy/8*100}%`,document.getElementById("test-min-val").textContent=s.min_entropy.toFixed(4),document.getElementById("test-min-bar").style.width=`${s.min_entropy/8*100}%`,document.getElementById("test-mean-val").textContent=s.mean.toFixed(3),document.getElementById("test-chi-val").textContent=s.chi_square.toFixed(2),document.getElementById("test-longest-run-val").textContent=s.longest_run.toString(),document.getElementById("test-score-val").textContent=s.overall_score.toFixed(1),t&&(t.innerHTML="",s.nist_results.forEach(r=>{let l=document.createElement("tr");l.className="border-b border-outline-variant/10 hover:bg-surface-container-low transition-colors group";let o=r.passed?'<span class="bg-[#00FF88]/10 text-[#00FF88] border border-[#00FF88]/30 px-3 py-1 font-bold">\u2713 PASS</span>':'<span class="bg-error-container/20 text-error border border-error/30 px-3 py-1 font-bold">\u2717 FAIL</span>';l.innerHTML=`
                            <td class="px-6 py-4 font-bold uppercase tracking-tight">${r.name}</td>
                            <td class="px-6 py-4 font-mono">${r.p_value.toFixed(6)}</td>
                            <td class="px-6 py-4 text-right">${o}</td>
                        `,t.appendChild(l)}))}catch(s){console.error(s),t&&(t.innerHTML=`<tr><td colspan="3" class="text-center text-error py-4">Error: ${s}</td></tr>`)}}));let j=document.getElementById("bench-size-slider"),oe=document.getElementById("bench-size-label"),we=document.getElementById("bench-run-btn"),Se=document.getElementById("bench-results-section");j&&oe&&(j.addEventListener("input",()=>{let e=parseFloat(j.value),n=Math.round(Math.pow(10,e));n>=1e6?oe.textContent=`${(n/1e6).toFixed(1)}M bytes`:n>=1e3?oe.textContent=`${(n/1e3).toFixed(0)}K bytes`:oe.textContent=`${n} bytes`}),j.dispatchEvent(new Event("input"))),we&&j&&Se&&we.addEventListener("click",()=>w(re,null,function*(){let e=parseFloat(j.value),n=Math.round(Math.pow(10,e));Se.classList.remove("hidden");try{let t=yield W("run_benchmark",{bytes:n});document.getElementById("bench-throughput").textContent=t.throughput_mbps.toFixed(2),document.getElementById("bench-latency").textContent=t.latency_us.toFixed(2);let s=t.bytes_generated.toLocaleString();document.getElementById("bench-details").textContent=`Generated ${s} bytes in ${t.duration_secs.toFixed(3)} seconds`}catch(t){console.error(t),document.getElementById("bench-details").textContent=`Error: ${t}`}}));let Te=document.querySelectorAll(".learn-btn"),Ue=document.querySelectorAll(".learn-section");Te.forEach(e=>{e.addEventListener("click",n=>{let t=n.currentTarget.getAttribute("data-learn");Te.forEach(s=>{s.classList.remove("active","bg-primary-container","text-on-primary-container"),s.classList.add("bg-surface-container","text-slate-400")}),n.currentTarget.classList.remove("bg-surface-container","text-slate-400"),n.currentTarget.classList.add("active","bg-primary-container","text-on-primary-container"),Ue.forEach(s=>{s.id===`learn-${t}`?s.classList.add("active"):s.classList.remove("active")})})});let h=[],x=0,B=null,M=document.getElementById("learn-input"),X=document.getElementById("learn-prev"),T=document.getElementById("learn-next"),A=document.getElementById("learn-play-pause"),ie=document.getElementById("learn-speed-slider"),ke=document.getElementById("learn-speed-text"),Fe=document.getElementById("learn-xor-step-info"),le=document.getElementById("learn-xor-content"),$e=document.getElementById("learn-progress-text"),Ce=document.getElementById("learn-progress-slider");function Be(){return w(this,null,function*(){if(M.value)try{h=(yield W("get_xor_steps",{text:M.value})).steps,x=0,B&&Z(),pe()}catch(e){console.error(e)}})}function pe(){if(h.length===0){le&&(le.innerHTML="No steps.");return}let e=h[x];Fe&&(Fe.textContent=`STEP_IDX: ${(x+1).toString().padStart(3,"0")} / ${h.length.toString().padStart(3,"0")}`);let n=Math.round((x+1)/h.length*100);if($e&&($e.textContent=`${n}%`),Ce&&(Ce.value=n.toString()),le){let t=e.input_binary.split("").map(l=>`<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-cyan-400">${l}</span>`).join(""),s=e.keystream_binary.split("").map(l=>`<span class="w-6 h-8 bg-surface-container-highest flex items-center justify-center font-bold text-orange-400">${l}</span>`).join(""),r=e.result_binary.split("").map(l=>`<span class="w-6 h-8 bg-primary-container/10 border border-green-400/30 flex items-center justify-center font-black text-green-400">${l}</span>`).join("");le.innerHTML=`
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
             `}Ve()}function Ve(){let e=document.getElementById("learn-xor-progress");if(!e)return;if(h.length===0){e.classList.add("hidden");return}e.classList.remove("hidden");let n=e.querySelector(".flex.flex-wrap.gap-2");if(!n)return;let t="";h.forEach((s,r)=>{let l=s.result_byte.toString(16).toUpperCase().padStart(2,"0"),o="px-3 py-2 rounded font-mono text-sm font-bold transition-colors ";r===x?o+="bg-green-500/30 border border-green-400 text-green-300":r<x?o+="bg-surface-container-highest text-slate-400 border border-outline-variant/20":o+="bg-surface-container text-slate-500 border border-outline-variant/20",t+=`<span class="${o}">${l}</span>`}),n.innerHTML=t}M&&(M.addEventListener("input",()=>{Be()}),setTimeout(Be,500)),X&&X.addEventListener("click",()=>{x>0&&(x--,pe())}),T&&T.addEventListener("click",()=>{x<h.length-1?(x++,pe()):B&&Z()});function Z(){if(!A)return;let e=A.querySelector(".material-symbols-outlined");if(B)clearInterval(B),B=null,e&&(e.textContent="play_arrow");else{x>=h.length-1&&(x=0);let n=parseInt(ie.value,10);B=setInterval(()=>{T&&T.click()},n),e&&(e.textContent="pause")}}A&&A.addEventListener("click",Z),ie&&ke&&ie.addEventListener("input",()=>{let e=(parseInt(ie.value,10)/1e3).toFixed(1);ke.textContent=`${e}s / step`,B&&(Z(),Z())});let E=document.getElementById("learn-xor-collapsible"),me=document.getElementById("learn-xor-details");E&&me&&E.addEventListener("click",()=>{me.classList.toggle("hidden");let e=!me.classList.contains("hidden");E.setAttribute("aria-expanded",e?"true":"false");let n=E.querySelector(".material-symbols-outlined");n&&(n.textContent=e?"expand_less":"expand_more")});let L=document.getElementById("learn-entropy-collapsible"),fe=document.getElementById("learn-entropy-details");L&&fe&&L.addEventListener("click",()=>{fe.classList.toggle("hidden");let e=!fe.classList.contains("hidden");L.setAttribute("aria-expanded",e?"true":"false");let n=L.querySelector(".material-symbols-outlined");n&&(n.textContent=e?"expand_less":"expand_more")});let _=document.getElementById("learn-nist-collapsible"),ye=document.getElementById("learn-nist-details");_&&ye&&_.addEventListener("click",()=>{ye.classList.toggle("hidden");let e=!ye.classList.contains("hidden");_.setAttribute("aria-expanded",e?"true":"false");let n=_.querySelector(".material-symbols-outlined");n&&(n.textContent=e?"expand_less":"expand_more")});let O=[],k=0,H=null,Y=document.getElementById("learn-entropy-input"),J=document.getElementById("learn-entropy-prev"),q=document.getElementById("learn-entropy-next"),F=document.getElementById("learn-entropy-play-pause"),Me=document.getElementById("learn-entropy-step-info"),Ae=document.getElementById("learn-entropy-content"),Q=document.getElementById("learn-entropy-calculate"),ce=document.getElementById("learn-entropy-speed-slider"),He=document.getElementById("learn-entropy-speed-text");function Ge(){return w(this,null,function*(){if(Y.value)try{O=(yield W("get_entropy_steps",{text:Y.value})).steps,k=0,H&&ee(),be()}catch(e){console.error(e)}})}function be(){var t,s;if(O.length===0)return;let e=O[k],n=e.step_type;if(Me&&(Me.textContent=`Step ${k+1} / ${O.length}`),Ae){let r=`<table class="w-full text-sm">
                 <thead>
                     <tr class="border-b border-outline-variant/30 text-left">
                         <th class="p-2 font-bold text-primary">Byte</th>
                         <th class="p-2 font-bold text-primary">Count</th>`;n!=="CountBytes"&&(r+='<th class="p-2 font-bold text-primary">Probability</th>'),(n==="CalculateContributions"||n==="SumEntropy"||n==="Interpret")&&(r+='<th class="p-2 font-bold text-primary">Contribution</th>'),r+=`<th class="p-2 font-bold text-primary">Visual</th>
                     </tr>
                 </thead>
                 <tbody>`;let l=Object.keys(e.byte_counts).sort(),o=Math.max(...Object.values(e.byte_counts),1);for(let m of l){let y=parseInt(m),g=e.byte_counts[m],ve=((t=e.probabilities)==null?void 0:t[m])||0,he=((s=e.entropy_contributions)==null?void 0:s[m])||0,Xe=y>=32&&y<=126?String.fromCharCode(y):`0x${y.toString(16).toUpperCase().padStart(2,"0")}`,Ze=g/o*100,Ye=g===1?"#00FF88":`rgb(255, ${100+(155-Math.min(g*20,155))}, 100)`;r+=`<tr class="border-b border-outline-variant/20 hover:bg-surface-container-low transition-colors">
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
                                  <div class="text-xs text-slate-500 mt-1">bits/byte (${l.length} unique)</div>
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
                      </div>`;break}}Ae.innerHTML=r+u}}function ee(){if(!F)return;let e=F.querySelector(".material-symbols-outlined"),n=F.querySelector("span:last-child");if(H)clearInterval(H),H=null,e&&(e.textContent="play_arrow"),n&&(n.textContent="Play");else{k>=O.length-1&&(k=0);let t=parseInt(ce.value,10);H=setInterval(()=>{q&&q.click()},t),e&&(e.textContent="pause"),n&&(n.textContent="Pause")}}Q&&Q.addEventListener("click",Ge),J&&J.addEventListener("click",()=>{k>0&&(k--,be())}),q&&q.addEventListener("click",()=>{k<O.length-1?(k++,be()):H&&ee()}),F&&F.addEventListener("click",ee),ce&&He&&ce.addEventListener("input",()=>{let e=(parseInt(ce.value,10)/1e3).toFixed(1);He.textContent=`${e}s / step`,H&&(ee(),ee())});let P=[],I=0,R=null,z=document.getElementById("learn-nist-input"),te=document.getElementById("learn-nist-analyze"),qe=document.getElementById("learn-nist-random"),ne=document.getElementById("learn-nist-prev"),D=document.getElementById("learn-nist-next"),$=document.getElementById("learn-nist-play-pause"),Pe=document.getElementById("learn-nist-step-info"),Re=document.getElementById("learn-nist-content"),de=document.getElementById("learn-nist-speed-slider"),De=document.getElementById("learn-nist-speed-text");function xe(e=!0,n=""){return w(this,null,function*(){try{let t;e?t=yield W("get_nist_steps_random",{count:64}):t=yield W("get_nist_steps",{text:n}),P=t.steps,I=0,R&&se(),ge()}catch(t){console.error(t)}})}function ge(){if(P.length===0)return;let e=P[I],n=e.step_type;if(Pe&&(Pe.textContent=`Step ${I+1} / ${P.length}`),Re){let t="";for(let u=0;u<e.bits.length;u++){u>0&&u%8===0&&(t+='<span class="mx-2"></span>');let m=e.bits[u];t+=`<span class="inline-block w-4 text-center font-bold ${m===1?"text-[#00FF88]":"text-slate-400"}">${m}</span>`}let s=`<div class="space-y-6">
                 <h3 class="text-lg font-bold text-[#A855F7]">Step ${I+1} of ${P.length}: ${n.replace(/([A-Z])/g," $1").trim()}</h3>

                 <div>
                     <h4 class="text-sm font-bold text-slate-300 mb-2">Bit Sequence (${e.bits.length} bits):</h4>
                     <div class="bg-surface-container-lowest p-4 rounded border border-outline-variant/20 font-mono text-sm break-all">
                         ${t}
                     </div>
                 </div>`,r=e.bits.length,l=r>0?e.ones_count/r*100:0;n!=="ConvertToBits"&&(s+=`<div class="grid grid-cols-2 gap-4">
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
                         <div class="absolute h-full bg-[#00FF88] transition-all" style="width: ${l}%;"></div>
                         <div class="absolute top-1/2 -translate-y-1/2 left-1/2 -translate-x-1/2 w-0.5 h-full bg-black"></div>
                         <div class="absolute inset-0 flex items-center justify-center font-bold text-sm text-white mix-blend-multiply">
                             ${l.toFixed(1)}% Ones (${e.ones_count}/${r} bits)
                         </div>
                     </div>
                 </div>`);let o="";switch(n){case"ConvertToBits":o=`<div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                         <p class="text-sm text-slate-300">
                             <strong>\u{1F4A1} What's happening:</strong> Each byte becomes 8 binary digits. White = 0, Green = 1.
                         </p>
                     </div>`;break;case"CountOnesZeros":o=`<div class="p-4 bg-surface-container-low rounded border-l-2 border-[#A855F7]">
                         <p class="text-sm text-slate-300 mb-2">
                             <strong>\u{1F4A1} What's happening:</strong> If the data is truly random, we expect roughly 50% ones and 50% zeros, like fair coin flips.
                         </p>
                         <p class="text-sm text-slate-400 italic">
                             Current ratio: ${l.toFixed(1)}% ones vs ${(100-l).toFixed(1)}% zeros
                         </p>
                     </div>`;break;case"CalculateStatistic":o=`<div class="space-y-3">
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
                     </div>`;break;case"CalculatePValue":o=`<div class="space-y-3">
                         <div class="bg-surface-container-low p-4 rounded border-l-2 border-[#A855F7]">
                             <p class="text-sm text-slate-300 mb-2">
                                 <strong>\u{1F4A1} Formula:</strong> P-value = erfc(S_obs \xF7 \u221A2)
                             </p>
                             <p class="text-sm text-slate-400">Result: erfc(${e.s_obs.toFixed(4)} \xF7 1.414) = ${e.p_value.toFixed(4)}</p>
                         </div>
                         <div class="p-3 bg-surface-container rounded text-xs text-slate-400">
                             <p><strong>Interpretation:</strong> P-value = ${(e.p_value*100).toFixed(1)}% probability that a truly random sequence would show this much deviation.</p>
                         </div>
                     </div>`;break;case"Interpret":{let u=e.passed?"#00FF88":"#ff6b6b",m=e.passed?"\u2705":"\u274C",y=e.passed?"PASS":"FAIL",g=e.passed?"The sequence looks RANDOM!":"The sequence is BIASED toward 1s or 0s.";o=`<div class="space-y-4">
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
                      </div>`;break}}s+=o+"</div>",Re.innerHTML=s}}function se(){if(!$)return;let e=$.querySelector(".material-symbols-outlined"),n=$.querySelector("span:last-child");if(R)clearInterval(R),R=null,e&&(e.textContent="play_arrow"),n&&(n.textContent="Play");else{I>=P.length-1&&(I=0);let t=parseInt(de.value,10);R=setInterval(()=>{D&&D.click()},t),e&&(e.textContent="pause"),n&&(n.textContent="Pause")}}te&&te.addEventListener("click",()=>{z.value&&xe(!1,z.value)}),qe&&qe.addEventListener("click",()=>{z.value="",xe(!0)}),ne&&ne.addEventListener("click",()=>{I>0&&(I--,ge())}),D&&D.addEventListener("click",()=>{I<P.length-1?(I++,ge()):R&&se()}),$&&$.addEventListener("click",se),de&&De&&de.addEventListener("input",()=>{let e=(parseInt(de.value,10)/1e3).toFixed(1);De.textContent=`${e}s / step`,R&&(se(),se())}),setTimeout(()=>xe(!0),500),d==null||d.addEventListener("keydown",e=>{e.key==="Enter"&&!e.shiftKey&&(e.preventDefault(),b==null||b.click())}),M==null||M.addEventListener("keydown",e=>{e.key==="Enter"&&(e.preventDefault(),T&&T.click())}),Y==null||Y.addEventListener("keydown",e=>{e.key==="Enter"&&(e.preventDefault(),Q==null||Q.click())}),z==null||z.addEventListener("keydown",e=>{e.key==="Enter"&&(e.preventDefault(),te==null||te.click())}),document.addEventListener("keydown",e=>{if(e.key==="Escape"){let n=document.getElementById("learn-xor-details"),t=document.getElementById("learn-entropy-details"),s=document.getElementById("learn-nist-details");if(n&&!n.classList.contains("hidden")){n.classList.add("hidden"),E==null||E.setAttribute("aria-expanded","false");let r=E==null?void 0:E.querySelector(".material-symbols-outlined");r&&(r.textContent="expand_more")}if(t&&!t.classList.contains("hidden")){t.classList.add("hidden"),L==null||L.setAttribute("aria-expanded","false");let r=L==null?void 0:L.querySelector(".material-symbols-outlined");r&&(r.textContent="expand_more")}if(s&&!s.classList.contains("hidden")){s.classList.add("hidden"),_==null||_.setAttribute("aria-expanded","false");let r=_==null?void 0:_.querySelector(".material-symbols-outlined");r&&(r.textContent="expand_more")}}}),document.addEventListener("keydown",e=>{let n=document.querySelector(".tab-panel.active");if(!(!n||n.id!=="tab-learn")){if(e.key==="ArrowRight"&&(e.ctrlKey||e.metaKey)){e.preventDefault();let t=n.querySelector(".learn-section.active");(t==null?void 0:t.id)==="learn-xor"?T==null||T.click():(t==null?void 0:t.id)==="learn-entropy"?q==null||q.click():(t==null?void 0:t.id)==="learn-nist"&&(D==null||D.click())}if(e.key==="ArrowLeft"&&(e.ctrlKey||e.metaKey)){e.preventDefault();let t=n.querySelector(".learn-section.active");(t==null?void 0:t.id)==="learn-xor"?X==null||X.click():(t==null?void 0:t.id)==="learn-entropy"?J==null||J.click():(t==null?void 0:t.id)==="learn-nist"&&(ne==null||ne.click())}if(e.key===" "&&e.ctrlKey){e.preventDefault();let t=n.querySelector(".learn-section.active");(t==null?void 0:t.id)==="learn-xor"?A==null||A.click():(t==null?void 0:t.id)==="learn-entropy"?F==null||F.click():(t==null?void 0:t.id)==="learn-nist"&&($==null||$.click())}}}),document.addEventListener("keydown",e=>{if(e.ctrlKey&&!e.shiftKey&&!e.altKey&&!e.metaKey){let n=parseInt(e.key,10);if(n>=1&&n<=4){e.preventDefault();let s={1:"tab-use",2:"tab-test",3:"tab-bench",4:"tab-learn"}[n];s&&c(s)}}})})});tt();})();
