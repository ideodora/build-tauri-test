function $(){}const Q=t=>t;function ht(t,n){for(const e in n)t[e]=n[e];return t}function U(t){return t()}function J(){return Object.create(null)}function k(t){t.forEach(U)}function M(t){return typeof t=="function"}function Rt(t,n){return t!=t?n==n:t!==n||t&&typeof t=="object"||typeof t=="function"}function mt(t){return Object.keys(t).length===0}function pt(t,...n){if(t==null)return $;const e=t.subscribe(...n);return e.unsubscribe?()=>e.unsubscribe():e}function zt(t,n,e){t.$$.on_destroy.push(pt(n,e))}function Bt(t,n,e,i){if(t){const r=V(t,n,e,i);return t[0](r)}}function V(t,n,e,i){return t[1]&&i?ht(e.ctx.slice(),t[1](i(n))):e.ctx}function qt(t,n,e,i){if(t[2]&&i){const r=t[2](i(e));if(n.dirty===void 0)return r;if(typeof r=="object"){const l=[],s=Math.max(n.dirty.length,r.length);for(let o=0;o<s;o+=1)l[o]=n.dirty[o]|r[o];return l}return n.dirty|r}return n.dirty}function Ft(t,n,e,i,r,l){if(r){const s=V(n,e,i,l);t.p(s,r)}}function Ht(t){if(t.ctx.length>32){const n=[],e=t.ctx.length/32;for(let i=0;i<e;i++)n[i]=-1;return n}return-1}function It(t){const n={};for(const e in t)e[0]!=="$"&&(n[e]=t[e]);return n}function Wt(t,n){const e={};n=new Set(n);for(const i in t)!n.has(i)&&i[0]!=="$"&&(e[i]=t[i]);return e}function Gt(t,n,e){return t.set(e),n}function Jt(t){return t&&M(t.destroy)?t.destroy:$}const X=typeof window<"u";let Y=X?()=>window.performance.now():()=>Date.now(),W=X?t=>requestAnimationFrame(t):$;const E=new Set;function Z(t){E.forEach(n=>{n.c(t)||(E.delete(n),n.f())}),E.size!==0&&W(Z)}function tt(t){let n;return E.size===0&&W(Z),{promise:new Promise(e=>{E.add(n={c:t,f:e})}),abort(){E.delete(n)}}}let z=!1;function yt(){z=!0}function gt(){z=!1}function $t(t,n,e,i){for(;t<n;){const r=t+(n-t>>1);e(r)<=i?t=r+1:n=r}return t}function bt(t){if(t.hydrate_init)return;t.hydrate_init=!0;let n=t.childNodes;if(t.nodeName==="HEAD"){const c=[];for(let u=0;u<n.length;u++){const _=n[u];_.claim_order!==void 0&&c.push(_)}n=c}const e=new Int32Array(n.length+1),i=new Int32Array(n.length);e[0]=-1;let r=0;for(let c=0;c<n.length;c++){const u=n[c].claim_order,_=(r>0&&n[e[r]].claim_order<=u?r+1:$t(1,r,h=>n[e[h]].claim_order,u))-1;i[c]=e[_]+1;const a=_+1;e[a]=c,r=Math.max(a,r)}const l=[],s=[];let o=n.length-1;for(let c=e[r]+1;c!=0;c=i[c-1]){for(l.push(n[c-1]);o>=c;o--)s.push(n[o]);o--}for(;o>=0;o--)s.push(n[o]);l.reverse(),s.sort((c,u)=>c.claim_order-u.claim_order);for(let c=0,u=0;c<s.length;c++){for(;u<l.length&&s[c].claim_order>=l[u].claim_order;)u++;const _=u<l.length?l[u]:null;t.insertBefore(s[c],_)}}function xt(t,n){t.appendChild(n)}function nt(t){if(!t)return document;const n=t.getRootNode?t.getRootNode():t.ownerDocument;return n&&n.host?n:t.ownerDocument}function wt(t){const n=it("style");return vt(nt(t),n),n.sheet}function vt(t,n){return xt(t.head||t,n),n.sheet}function Et(t,n){if(z){for(bt(t),(t.actual_end_child===void 0||t.actual_end_child!==null&&t.actual_end_child.parentNode!==t)&&(t.actual_end_child=t.firstChild);t.actual_end_child!==null&&t.actual_end_child.claim_order===void 0;)t.actual_end_child=t.actual_end_child.nextSibling;n!==t.actual_end_child?(n.claim_order!==void 0||n.parentNode!==t)&&t.insertBefore(n,t.actual_end_child):t.actual_end_child=n.nextSibling}else(n.parentNode!==t||n.nextSibling!==null)&&t.appendChild(n)}function Kt(t,n,e){z&&!e?Et(t,n):(n.parentNode!==t||n.nextSibling!=e)&&t.insertBefore(n,e||null)}function et(t){t.parentNode&&t.parentNode.removeChild(t)}function Qt(t,n){for(let e=0;e<t.length;e+=1)t[e]&&t[e].d(n)}function it(t){return document.createElement(t)}function kt(t){return document.createElementNS("http://www.w3.org/2000/svg",t)}function G(t){return document.createTextNode(t)}function Ut(){return G(" ")}function Vt(){return G("")}function Xt(t,n,e,i){return t.addEventListener(n,e,i),()=>t.removeEventListener(n,e,i)}function Yt(t){return function(n){return n.preventDefault(),t.call(this,n)}}function Zt(t){return function(n){return n.stopPropagation(),t.call(this,n)}}function rt(t,n,e){e==null?t.removeAttribute(n):t.getAttribute(n)!==e&&t.setAttribute(n,e)}function tn(t,n){const e=Object.getOwnPropertyDescriptors(t.__proto__);for(const i in n)n[i]==null?t.removeAttribute(i):i==="style"?t.style.cssText=n[i]:i==="__value"?t.value=t[i]=n[i]:e[i]&&e[i].set?t[i]=n[i]:rt(t,i,n[i])}function nn(t,n){for(const e in n)rt(t,e,n[e])}function en(t){return t===""?null:+t}function Nt(t){return Array.from(t.childNodes)}function jt(t){t.claim_info===void 0&&(t.claim_info={last_index:0,total_claimed:0})}function st(t,n,e,i,r=!1){jt(t);const l=(()=>{for(let s=t.claim_info.last_index;s<t.length;s++){const o=t[s];if(n(o)){const c=e(o);return c===void 0?t.splice(s,1):t[s]=c,r||(t.claim_info.last_index=s),o}}for(let s=t.claim_info.last_index-1;s>=0;s--){const o=t[s];if(n(o)){const c=e(o);return c===void 0?t.splice(s,1):t[s]=c,r?c===void 0&&t.claim_info.last_index--:t.claim_info.last_index=s,o}}return i()})();return l.claim_order=t.claim_info.total_claimed,t.claim_info.total_claimed+=1,l}function ot(t,n,e,i){return st(t,r=>r.nodeName===n,r=>{const l=[];for(let s=0;s<r.attributes.length;s++){const o=r.attributes[s];e[o.name]||l.push(o.name)}l.forEach(s=>r.removeAttribute(s))},()=>i(n))}function rn(t,n,e){return ot(t,n,e,it)}function sn(t,n,e){return ot(t,n,e,kt)}function At(t,n){return st(t,e=>e.nodeType===3,e=>{const i=""+n;if(e.data.startsWith(i)){if(e.data.length!==i.length)return e.splitText(i.length)}else e.data=i},()=>G(n),!0)}function on(t){return At(t," ")}function cn(t,n){n=""+n,t.wholeText!==n&&(t.data=n)}function ln(t,n){t.value=n??""}function un(t,n,e,i){e===null?t.style.removeProperty(n):t.style.setProperty(n,e,i?"important":"")}function an(t,n,e){t.classList[e?"add":"remove"](n)}function ct(t,n,{bubbles:e=!1,cancelable:i=!1}={}){const r=document.createEvent("CustomEvent");return r.initCustomEvent(t,e,i,n),r}function fn(t,n){return new t(n)}const L=new Map;let R=0;function Ct(t){let n=5381,e=t.length;for(;e--;)n=(n<<5)-n^t.charCodeAt(e);return n>>>0}function St(t,n){const e={stylesheet:wt(n),rules:{}};return L.set(t,e),e}function q(t,n,e,i,r,l,s,o=0){const c=16.666/i;let u=`{
`;for(let p=0;p<=1;p+=c){const y=n+(e-n)*l(p);u+=p*100+`%{${s(y,1-y)}}
`}const _=u+`100% {${s(e,1-e)}}
}`,a=`__svelte_${Ct(_)}_${o}`,h=nt(t),{stylesheet:f,rules:d}=L.get(h)||St(h,t);d[a]||(d[a]=!0,f.insertRule(`@keyframes ${a} ${_}`,f.cssRules.length));const m=t.style.animation||"";return t.style.animation=`${m?`${m}, `:""}${a} ${i}ms linear ${r}ms 1 both`,R+=1,a}function F(t,n){const e=(t.style.animation||"").split(", "),i=e.filter(n?l=>l.indexOf(n)<0:l=>l.indexOf("__svelte")===-1),r=e.length-i.length;r&&(t.style.animation=i.join(", "),R-=r,R||Dt())}function Dt(){W(()=>{R||(L.forEach(t=>{const{ownerNode:n}=t.stylesheet;n&&et(n)}),L.clear())})}let D;function C(t){D=t}function N(){if(!D)throw new Error("Function called outside component initialization");return D}function _n(t){N().$$.on_mount.push(t)}function dn(t){N().$$.after_update.push(t)}function hn(t){N().$$.on_destroy.push(t)}function mn(){const t=N();return(n,e,{cancelable:i=!1}={})=>{const r=t.$$.callbacks[n];if(r){const l=ct(n,e,{cancelable:i});return r.slice().forEach(s=>{s.call(t,l)}),!l.defaultPrevented}return!0}}function pn(t,n){return N().$$.context.set(t,n),n}function yn(t){return N().$$.context.get(t)}function gn(t,n){const e=t.$$.callbacks[n.type];e&&e.slice().forEach(i=>i.call(this,n))}const v=[],K=[],O=[],H=[],lt=Promise.resolve();let I=!1;function ut(){I||(I=!0,lt.then(at))}function $n(){return ut(),lt}function P(t){O.push(t)}function bn(t){H.push(t)}const B=new Set;let w=0;function at(){if(w!==0)return;const t=D;do{try{for(;w<v.length;){const n=v[w];w++,C(n),Pt(n.$$)}}catch(n){throw v.length=0,w=0,n}for(C(null),v.length=0,w=0;K.length;)K.pop()();for(let n=0;n<O.length;n+=1){const e=O[n];B.has(e)||(B.add(e),e())}O.length=0}while(v.length);for(;H.length;)H.pop()();I=!1,B.clear(),C(t)}function Pt(t){if(t.fragment!==null){t.update(),k(t.before_update);const n=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,n),t.after_update.forEach(P)}}let A;function ft(){return A||(A=Promise.resolve(),A.then(()=>{A=null})),A}function S(t,n,e){t.dispatchEvent(ct(`${n?"intro":"outro"}${e}`))}const T=new Set;let g;function xn(){g={r:0,c:[],p:g}}function wn(){g.r||k(g.c),g=g.p}function Mt(t,n){t&&t.i&&(T.delete(t),t.i(n))}function vn(t,n,e,i){if(t&&t.o){if(T.has(t))return;T.add(t),g.c.push(()=>{T.delete(t),i&&(e&&t.d(1),i())}),t.o(n)}else i&&i()}const _t={duration:0};function En(t,n,e){const i={direction:"in"};let r=n(t,e,i),l=!1,s,o,c=0;function u(){s&&F(t,s)}function _(){const{delay:h=0,duration:f=300,easing:d=Q,tick:m=$,css:p}=r||_t;p&&(s=q(t,0,1,f,h,d,p,c++)),m(0,1);const y=Y()+h,j=y+f;o&&o.abort(),l=!0,P(()=>S(t,!0,"start")),o=tt(b=>{if(l){if(b>=j)return m(1,0),S(t,!0,"end"),u(),l=!1;if(b>=y){const x=d((b-y)/f);m(x,1-x)}}return l})}let a=!1;return{start(){a||(a=!0,F(t),M(r)?(r=r(i),ft().then(_)):_())},invalidate(){a=!1},end(){l&&(u(),l=!1)}}}function kn(t,n,e,i){const r={direction:"both"};let l=n(t,e,r),s=i?0:1,o=null,c=null,u=null;function _(){u&&F(t,u)}function a(f,d){const m=f.b-s;return d*=Math.abs(m),{a:s,b:f.b,d:m,duration:d,start:f.start,end:f.start+d,group:f.group}}function h(f){const{delay:d=0,duration:m=300,easing:p=Q,tick:y=$,css:j}=l||_t,b={start:Y()+d,b:f};f||(b.group=g,g.r+=1),o||c?c=b:(j&&(_(),u=q(t,s,f,m,d,p,j)),f&&y(0,1),o=a(b,m),P(()=>S(t,f,"start")),tt(x=>{if(c&&x>c.start&&(o=a(c,m),c=null,S(t,o.b,"start"),j&&(_(),u=q(t,s,o.b,o.duration,0,p,l.css))),o){if(x>=o.end)y(s=o.b,1-s),S(t,o.b,"end"),c||(o.b?_():--o.group.r||k(o.group.c)),o=null;else if(x>=o.start){const dt=x-o.start;s=o.a+o.d*p(dt/o.duration),y(s,1-s)}}return!!(o||c)}))}return{run(f){M(l)?ft().then(()=>{l=l(r),h(f)}):h(f)},end(){_(),o=c=null}}}const Nn=typeof window<"u"?window:typeof globalThis<"u"?globalThis:global;function jn(t,n){const e={},i={},r={$$scope:1};let l=t.length;for(;l--;){const s=t[l],o=n[l];if(o){for(const c in s)c in o||(i[c]=1);for(const c in o)r[c]||(e[c]=o[c],r[c]=1);t[l]=o}else for(const c in s)r[c]=1}for(const s in i)s in e||(e[s]=void 0);return e}function An(t){return typeof t=="object"&&t!==null?t:{}}function Cn(t,n,e){const i=t.$$.props[n];i!==void 0&&(t.$$.bound[i]=e,e(t.$$.ctx[i]))}function Sn(t){t&&t.c()}function Dn(t,n){t&&t.l(n)}function Ot(t,n,e,i){const{fragment:r,after_update:l}=t.$$;r&&r.m(n,e),i||P(()=>{const s=t.$$.on_mount.map(U).filter(M);t.$$.on_destroy?t.$$.on_destroy.push(...s):k(s),t.$$.on_mount=[]}),l.forEach(P)}function Tt(t,n){const e=t.$$;e.fragment!==null&&(k(e.on_destroy),e.fragment&&e.fragment.d(n),e.on_destroy=e.fragment=null,e.ctx=[])}function Lt(t,n){t.$$.dirty[0]===-1&&(v.push(t),ut(),t.$$.dirty.fill(0)),t.$$.dirty[n/31|0]|=1<<n%31}function Pn(t,n,e,i,r,l,s,o=[-1]){const c=D;C(t);const u=t.$$={fragment:null,ctx:[],props:l,update:$,not_equal:r,bound:J(),on_mount:[],on_destroy:[],on_disconnect:[],before_update:[],after_update:[],context:new Map(n.context||(c?c.$$.context:[])),callbacks:J(),dirty:o,skip_bound:!1,root:n.target||c.$$.root};s&&s(u.root);let _=!1;if(u.ctx=e?e(t,n.props||{},(a,h,...f)=>{const d=f.length?f[0]:h;return u.ctx&&r(u.ctx[a],u.ctx[a]=d)&&(!u.skip_bound&&u.bound[a]&&u.bound[a](d),_&&Lt(t,a)),h}):[],u.update(),_=!0,k(u.before_update),u.fragment=i?i(u.ctx):!1,n.target){if(n.hydrate){yt();const a=Nt(n.target);u.fragment&&u.fragment.l(a),a.forEach(et)}else u.fragment&&u.fragment.c();n.intro&&Mt(t.$$.fragment),Ot(t,n.target,n.anchor,n.customElement),gt(),at()}C(c)}class Mn{$destroy(){Tt(this,1),this.$destroy=$}$on(n,e){if(!M(e))return $;const i=this.$$.callbacks[n]||(this.$$.callbacks[n]=[]);return i.push(e),()=>{const r=i.indexOf(e);r!==-1&&i.splice(r,1)}}$set(n){this.$$set&&!mt(n)&&(this.$$.skip_bound=!0,this.$$set(n),this.$$.skip_bound=!1)}}export{yn as $,$n as A,$ as B,Bt as C,Ft as D,Ht as E,qt as F,Et as G,zt as H,ht as I,kt as J,sn as K,nn as L,jn as M,Wt as N,It as O,Q as P,ln as Q,Xt as R,Mn as S,k as T,mn as U,P as V,kn as W,Qt as X,an as Y,en as Z,Gt as _,Ut as a,pn as a0,K as a1,hn as a2,Nn as a3,Jt as a4,M as a5,gn as a6,Yt as a7,Zt as a8,N as a9,Cn as aa,An as ab,bn as ac,tn as ad,En as ae,Kt as b,on as c,wn as d,Vt as e,Mt as f,xn as g,et as h,Pn as i,dn as j,it as k,rn as l,Nt as m,rt as n,_n as o,un as p,G as q,At as r,Rt as s,vn as t,cn as u,fn as v,Sn as w,Dn as x,Ot as y,Tt as z};