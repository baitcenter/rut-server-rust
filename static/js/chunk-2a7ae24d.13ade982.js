(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-2a7ae24d"],{"0d83":function(t,r,n){"use strict";var e=function(){var t=this,r=t.$createElement,n=t._self._c||r;return n("div",["0"!==t.action?n("b",[t._v(t._s(t.action.toUpperCase())+": "+t._s(t.totalCount))]):t._e(),n("div",{staticClass:"rut-list"},t._l(t.ruts,function(t){return n("rut-sum",{key:t.id,attrs:{rut:t}})}),1),t.hasMore?n("div",[n("el-button",{attrs:{size:"mini",disabled:!t.hasMore},on:{click:t.loadMoreRut}},[t._v("\n      Show More\n    ")])],1):t._e()])},i=[],o=n("75fc"),a=n("1199"),s=n("d722"),u={name:"rut-list",props:{per:String,action:{type:String,default:"0"},id:String},components:{RutSum:a["a"]},data:function(){return{perid:"",totalCount:0,ruts:[],paging:1}},computed:{hasMore:function(){return this.ruts.length<this.totalCount}},methods:{loadRuts:function(){var t=this,r=this.perid=this.id?this.id:this.$route.params.id;Object(s["n"])(this.per,r,this.paging,this.action).then(function(r){t.ruts=r.data.ruts,t.$store.commit("SET_RUTS",t.ruts),t.totalCount=r.data.count})},loadMoreRut:function(){var t=this;Object(s["n"])(this.per,this.perid,this.paging+1,this.action).then(function(r){var n,e=r.data.ruts;t.$store.commit("SET_RUTS",e),(n=t.ruts).push.apply(n,Object(o["a"])(e)),t.paging+=1})}},created:function(){this.loadRuts()}},c=u,f=(n("515f"),n("2877")),l=Object(f["a"])(c,e,i,!1,null,"306b1c07",null);r["a"]=l.exports},1199:function(t,r,n){"use strict";var e=function(){var t=this,r=t.$createElement,n=t._self._c||r;return n("div",{staticClass:"rut-sum"},[n("span",{staticClass:"title"},[t.rut.url?[t._v("\n      "+t._s(t.rut.title)+"\n      "),n("span",{staticClass:"host"},[n("a",{attrs:{href:t.rut.url,target:"_blank",rel:"nofollow noopener noreferrer"}},[t._v("\n          ("+t._s(t._f("host")(t.rut.url))+")\n        ")])])]:[n("router-link",{attrs:{to:"/r/"+t.rut.id}},[t._v("\n        "+t._s(t.rut.title)+"\n      ")])]],2),n("router-link",{attrs:{to:"/r/"+t.rut.id}},[n("span",[n("img",{staticClass:"cover",attrs:{src:t.rut.logo,referrerPolicy:"no-referrer"}})]),n("div",{staticClass:"content",domProps:{innerHTML:t._s(t.content)}}),t.rut.item_count>0?n("span",{staticClass:"meta"},[t._v("\n      including "+t._s(t._f("pluralise")(t.rut.item_count,"item"))+"  \n    ")]):n("span",{staticClass:"meta"},[n("router-link",{attrs:{to:t.to_url}},[t._v("...view")])],1)])],1)},i=[],o=n("5ad4"),a=n("e6d6"),s={name:"rut-sum",props:["rut"],computed:{content:function(){var t=Object(a["a"])(this.rut.content);return Object(o["showLess"])(t)},to_url:function(){return this.rut.content?"/r/"+this.rut.id:"/rforum/"+this.rut.id}}},u=s,c=(n("63a9"),n("2877")),f=Object(c["a"])(u,e,i,!1,null,"9f39eb5a",null);r["a"]=f.exports},"1af6":function(t,r,n){var e=n("63b6");e(e.S,"Array",{isArray:n("9003")})},"20fd":function(t,r,n){"use strict";var e=n("d9f6"),i=n("aebd");t.exports=function(t,r,n){r in t?e.f(t,r,i(0,n)):t[r]=n}},"277e":function(t,r,n){"use strict";n.r(r),n.d(r,"default",function(){return i});var e=n("0d83");function i(t,r,n){return{name:"".concat(t,"-ruts"),render:function(i){return i(e["a"],{props:{per:t,action:r,id:n}})}}}},"40fe":function(t,r,n){},"515f":function(t,r,n){"use strict";var e=n("6182"),i=n.n(e);i.a},"549b":function(t,r,n){"use strict";var e=n("d864"),i=n("63b6"),o=n("241e"),a=n("b0dc"),s=n("3702"),u=n("b447"),c=n("20fd"),f=n("7cd6");i(i.S+i.F*!n("4ee1")(function(t){Array.from(t)}),"Array",{from:function(t){var r,n,i,l,d=o(t),p="function"==typeof this?this:Array,h=arguments.length,v=h>1?arguments[1]:void 0,b=void 0!==v,m=0,_=f(d);if(b&&(v=e(v,h>2?arguments[2]:void 0,2)),void 0==_||p==Array&&s(_))for(r=u(d.length),n=new p(r);r>m;m++)c(n,m,b?v(d[m],m):d[m]);else for(l=_.call(d),n=new p;!(i=l.next()).done;m++)c(n,m,b?a(l,v,[i.value,m],!0):i.value);return n.length=m,n}})},"54a1":function(t,r,n){n("6c1c"),n("1654"),t.exports=n("95d5")},6182:function(t,r,n){},"63a9":function(t,r,n){"use strict";var e=n("40fe"),i=n.n(e);i.a},"75fc":function(t,r,n){"use strict";var e=n("a745"),i=n.n(e);function o(t){if(i()(t)){for(var r=0,n=new Array(t.length);r<t.length;r++)n[r]=t[r];return n}}var a=n("774e"),s=n.n(a),u=n("c8bb"),c=n.n(u);function f(t){if(c()(Object(t))||"[object Arguments]"===Object.prototype.toString.call(t))return s()(t)}function l(){throw new TypeError("Invalid attempt to spread non-iterable instance")}function d(t){return o(t)||f(t)||l()}n.d(r,"a",function(){return d})},"774e":function(t,r,n){t.exports=n("d2d5")},"95d5":function(t,r,n){var e=n("40c3"),i=n("5168")("iterator"),o=n("481b");t.exports=n("584a").isIterable=function(t){var r=Object(t);return void 0!==r[i]||"@@iterator"in r||o.hasOwnProperty(e(r))}},a745:function(t,r,n){t.exports=n("f410")},c8bb:function(t,r,n){t.exports=n("54a1")},d2d5:function(t,r,n){n("1654"),n("549b"),t.exports=n("584a").Array.from},f410:function(t,r,n){n("1af6"),t.exports=n("584a").Array.isArray}}]);
//# sourceMappingURL=chunk-2a7ae24d.13ade982.js.map