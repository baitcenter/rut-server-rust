(window["webpackJsonp"]=window["webpackJsonp"]||[]).push([["chunk-486a8857"],{1199:function(t,n,e){"use strict";var r=function(){var t=this,n=t.$createElement,e=t._self._c||n;return e("div",{staticClass:"rut-sum"},[e("span",{staticClass:"title"},[t.rut.url?[t._v("\n      "+t._s(t.rut.title)+"\n      "),e("span",{staticClass:"host"},[e("a",{attrs:{href:t.rut.url,target:"_blank",rel:"nofollow noopener noreferrer"}},[t._v("\n          ("+t._s(t._f("host")(t.rut.url))+")\n        ")])])]:[e("router-link",{attrs:{to:"/r/"+t.rut.id}},[t._v("\n        "+t._s(t.rut.title)+"\n      ")])]],2),e("router-link",{attrs:{to:"/r/"+t.rut.id}},[e("span",[e("img",{staticClass:"cover",attrs:{src:t.rut.logo,referrerPolicy:"no-referrer"}})]),e("div",{staticClass:"content",domProps:{innerHTML:t._s(t.content)}}),t.rut.item_count>0?e("span",{staticClass:"meta"},[t._v("\n      including "+t._s(t._f("pluralise")(t.rut.item_count,"item"))+"  \n    ")]):e("span",{staticClass:"meta"},[e("router-link",{attrs:{to:t.to_url}},[t._v("...view")])],1)])],1)},s=[],i=e("5ad4"),u=e("e6d6"),a={name:"rut-sum",props:["rut"],computed:{content:function(){var t=Object(u["a"])(this.rut.content);return Object(i["showLess"])(t)},to_url:function(){return this.rut.content?"/r/"+this.rut.id:"/rforum/"+this.rut.id}}},o=a,c=(e("63a9"),e("2877")),l=Object(c["a"])(o,r,s,!1,null,"9f39eb5a",null);n["a"]=l.exports},"40fe":function(t,n,e){},"63a9":function(t,n,e){"use strict";var r=e("40fe"),s=e.n(r);s.a},"663b":function(t,n,e){"use strict";var r=e("9606"),s=e.n(r);s.a},9606:function(t,n,e){},bb51:function(t,n,e){"use strict";e.r(n);var r=function(){var t=this,n=t.$createElement,e=t._self._c||n;return e("div",[e("div",{staticClass:"home-page"},[e("div",{staticClass:"rut-list"},t._l(t.indexRuts,function(t){return e("rut-sum",{key:t.id,attrs:{rut:t}})}),1),t._m(0)])])},s=[function(){var t=this,n=t.$createElement,e=t._self._c||n;return e("div",{staticClass:"home-side"},[e("small",[t._v("Yet Another Collection")])])}],i=e("1199"),u={name:"home",title:"Home",components:{RutSum:i["a"]},data:function(){return{indexRuts:[]}},methods:{loadIndex:function(){var t=this;this.$store.dispatch("getIndexRuts").then(function(n){t.indexRuts=n})}},created:function(){this.loadIndex()}},a=u,o=(e("663b"),e("2877")),c=Object(o["a"])(a,r,s,!1,null,"7a294401",null);n["default"]=c.exports}}]);
//# sourceMappingURL=chunk-486a8857.7ca35ebb.js.map