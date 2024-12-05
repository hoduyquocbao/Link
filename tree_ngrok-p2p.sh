#!/bin/bash

# Tạo thư mục gốc
mkdir -p ngrok-p2p
cd ngrok-p2p

# Tạo file gốc và cấu hình
touch Cargo.toml README.md
mkdir -p .github/workflows
touch .github/workflows/{ci.yml,cd.yml,security.yml,quality.yml}
mkdir -p .config/{development,staging,production}
touch .config/development/{app.toml,logging.toml,security.toml}
touch .config/staging/{app.toml,logging.toml,security.toml}
touch .config/production/{app.toml,logging.toml,security.toml}

# Tạo cấu trúc docs chi tiết
mkdir -p docs/{architecture,api,protocols,deployment,security,development,operations,research}/{design,implementation,validation}
touch docs/architecture/design/{system,network,data,security,scalability,reliability}.md
touch docs/architecture/implementation/{components,interfaces,patterns,practices}.md
touch docs/architecture/validation/{requirements,constraints,assumptions,risks}.md

touch docs/api/design/{rest,grpc,websocket,internal,public,private}.md
touch docs/api/implementation/{endpoints,handlers,middleware,security}.md
touch docs/api/validation/{testing,monitoring,documentation,versioning}.md

touch docs/protocols/design/{p2p,tunnel,control,discovery,relay,handshake}.md
touch docs/protocols/implementation/{flow,states,messages,encoding}.md
touch docs/protocols/validation/{conformance,interoperability,security,performance}.md

# Tạo cấu trúc crates với chiều sâu
mkdir -p crates

# Core crate - Nhân hệ thống
mkdir -p crates/core/src/{kernel,runtime,system,process,thread,memory,network,device}
touch crates/core/Cargo.toml
touch crates/core/src/lib.rs
touch crates/core/src/kernel/{scheduler,allocator,dispatcher,interrupt,syscall,context}.rs
touch crates/core/src/runtime/{executor,reactor,poller,timer,signal,resource}.rs
touch crates/core/src/system/{config,state,event,monitor,control,resource}.rs
touch crates/core/src/process/{manager,scheduler,container,isolation,lifecycle,resource}.rs
touch crates/core/src/thread/{pool,scheduler,worker,context,local,sync}.rs
touch crates/core/src/memory/{allocator,pool,cache,gc,mapping,protection}.rs
touch crates/core/src/network/{stack,protocol,socket,packet,stream,interface}.rs
touch crates/core/src/device/{manager,driver,bus,interrupt,io,resource}.rs

# Network crate - Xử lý mạng
mkdir -p crates/network/src/{transport,protocol,socket,packet,stream,connection,routing,qos}
touch crates/network/Cargo.toml
touch crates/network/src/lib.rs
touch crates/network/src/transport/{tcp,udp,quic,sctp,raw,unix}.rs
touch crates/network/src/protocol/{ip,icmp,arp,dns,dhcp,ntp}.rs
touch crates/network/src/socket/{stream,dgram,raw,listener,acceptor,connector}.rs
touch crates/network/src/packet/{header,payload,fragment,reassembly,filter,queue}.rs
touch crates/network/src/stream/{buffer,window,congestion,flow,multiplexer,demultiplexer}.rs
touch crates/network/src/connection/{manager,pool,monitor,limiter,tracker,analyzer}.rs
touch crates/network/src/routing/{table,policy,filter,nat,vpn,tunnel}.rs
touch crates/network/src/qos/{scheduler,shaper,policer,classifier,marker,monitor}.rs

# P2P crate - Xử lý ngang hàng
mkdir -p crates/p2p/src/{discovery,routing,transport,protocol,security,storage,sync,network}
touch crates/p2p/Cargo.toml
touch crates/p2p/src/lib.rs
touch crates/p2p/src/discovery/{dht,mdns,upnp,bootstrap,cache,resolver}.rs
touch crates/p2p/src/routing/{kademlia,chord,pastry,tapestry,can,symphony}.rs
touch crates/p2p/src/transport/{reliable,unreliable,ordered,unordered,encrypted,compressed}.rs
touch crates/p2p/src/protocol/{handshake,exchange,relay,broadcast,multicast,unicast}.rs
touch crates/p2p/src/security/{identity,trust,reputation,blacklist,whitelist,firewall}.rs
touch crates/p2p/src/storage/{dht,cache,persistent,temporary,distributed,replicated}.rs
touch crates/p2p/src/sync/{state,merkle,vector,causal,total,partial}.rs
touch crates/p2p/src/network/{overlay,underlay,virtual,physical,mesh,tree}.rs

# NAT crate - Xử lý NAT Traversal
mkdir -p crates/nat/src/{traversal,mapping,filtering,detection,relay,protocol}
touch crates/nat/Cargo.toml
touch crates/nat/src/lib.rs
touch crates/nat/src/traversal/{stun,turn,ice,upnp,pcp,nat_pmp}.rs
touch crates/nat/src/mapping/{endpoint,binding,allocation,reservation,timeout}.rs
touch crates/nat/src/filtering/{address,port,protocol,payload,state,behavior}.rs
touch crates/nat/src/detection/{type,behavior,capability,restriction,topology}.rs
touch crates/nat/src/relay/{server,client,protocol,session,stream,datagram}.rs
touch crates/nat/src/protocol/{stun,turn,ice,upnp,pcp,nat_pmp}.rs

# Tunnel crate - Xử lý tunnel
mkdir -p crates/tunnel/src/{transport,protocol,encryption,compression,routing,qos}
touch crates/tunnel/Cargo.toml
touch crates/tunnel/src/lib.rs
touch crates/tunnel/src/transport/{tcp,udp,quic,websocket,raw,multiplex}.rs
touch crates/tunnel/src/protocol/{handshake,control,data,error,keepalive,metadata}.rs
touch crates/tunnel/src/encryption/{tls,dtls,noise,wireguard,ipsec,custom}.rs
touch crates/tunnel/src/compression/{gzip,zlib,lz4,snappy,brotli,custom}.rs
touch crates/tunnel/src/routing/{policy,filter,nat,vpn,proxy,gateway}.rs
touch crates/tunnel/src/qos/{bandwidth,latency,priority,fairness,congestion,reliability}.rs

# Proxy crate - Xử lý proxy
mkdir -p crates/proxy/src/{protocol,filter,router,balancer,cache,security}
touch crates/proxy/Cargo.toml
touch crates/proxy/src/lib.rs
touch crates/proxy/src/protocol/{http,socks4,socks5,transparent,reverse,forward}.rs
touch crates/proxy/src/filter/{request,response,content,header,body,trailer}.rs
touch crates/proxy/src/router/{policy,rule,table,static,dynamic,custom}.rs
touch crates/proxy/src/balancer/{round_robin,least_conn,weighted,hash,random,custom}.rs
touch crates/proxy/src/cache/{memory,disk,distributed,policy,invalidation,replication}.rs
touch crates/proxy/src/security/{auth,acl,firewall,inspection,mitm,custom}.rs

# Metrics crate - Thu thập metrics
mkdir -p crates/metrics/src/{collector,aggregator,processor,storage,export,analyze}
touch crates/metrics/Cargo.toml
touch crates/metrics/src/lib.rs
touch crates/metrics/src/collector/{system,process,network,custom,push,pull}.rs
touch crates/metrics/src/aggregator/{counter,gauge,histogram,summary,cardinality,custom}.rs
touch crates/metrics/src/processor/{filter,transform,enrich,sample,batch,stream}.rs
touch crates/metrics/src/storage/{memory,disk,tsdb,warehouse,retention,backup}.rs
touch crates/metrics/src/export/{prometheus,statsd,influx,graphite,custom,format}.rs
touch crates/metrics/src/analyze/{query,aggregate,correlate,predict,alert,report}.rs

# Tracing crate - Distributed tracing
mkdir -p crates/tracing/src/{tracer,span,event,context,export,analyze}
touch crates/tracing/Cargo.toml
touch crates/tracing/src/lib.rs
touch crates/tracing/src/tracer/{provider,sampler,recorder,processor,exporter}.rs
touch crates/tracing/src/span/{context,attribute,link,event,status,timing}.rs
touch crates/tracing/src/event/{log,metric,trace,span,error,custom}.rs
touch crates/tracing/src/context/{propagation,extraction,injection,baggage,scope}.rs
touch crates/tracing/src/export/{jaeger,zipkin,otlp,custom,format,batch}.rs
touch crates/tracing/src/analyze/{query,filter,aggregate,correlate,visualize}.rs

# Discovery crate - Service discovery
mkdir -p crates/discovery/src/{registry,resolver,cache,health,load_balancer,failover}
touch crates/discovery/Cargo.toml
touch crates/discovery/src/lib.rs
touch crates/discovery/src/registry/{service,endpoint,instance,metadata,event,store}.rs
touch crates/discovery/src/resolver/{dns,mdns,consul,etcd,zookeeper,custom}.rs
touch crates/discovery/src/cache/{memory,disk,distributed,policy,invalidation,sync}.rs
touch crates/discovery/src/health/{check,probe,monitor,status,threshold,action}.rs
touch crates/discovery/src/load_balancer/{strategy,policy,session,sticky,weight,metric}.rs
touch crates/discovery/src/failover/{detection,recovery,backup,restore,switch,verify}.rs

# Circuit crate - Circuit breaking
mkdir -p crates/circuit/src/{breaker,state,metric,policy,notification,recovery}
touch crates/circuit/Cargo.toml
touch crates/circuit/src/lib.rs
touch crates/circuit/src/breaker/{threshold,window,counter,timer,state,transition}.rs
touch crates/circuit/src/state/{open,closed,half_open,forced,monitor,history}.rs
touch crates/circuit/src/metric/{success,failure,timeout,rejection,latency,custom}.rs
touch crates/circuit/src/policy/{threshold,ratio,latency,custom,composite,adaptive}.rs
touch crates/circuit/src/notification/{event,listener,handler,channel,filter,router}.rs
touch crates/circuit/src/recovery/{strategy,backoff,retry,timeout,fallback,verify}.rs

# Rate crate - Rate limiting
mkdir -p crates/rate/src/{limiter,bucket,window,policy,storage,notification}
touch crates/rate/Cargo.toml
touch crates/rate/src/lib.rs
touch crates/rate/src/limiter/{token,leaky,window,adaptive,distributed,custom}.rs
touch crates/rate/src/bucket/{fixed,sliding,adaptive,composite,overflow,underflow}.rs
touch crates/rate/src/window/{fixed,sliding,tumbling,hopping,session,custom}.rs
touch crates/rate/src/policy/{rate,burst,quota,penalty,throttle,backoff}.rs
touch crates/rate/src/storage/{memory,redis,database,cluster,sync,backup}.rs
touch crates/rate/src/notification/{event,listener,handler,channel,filter,router}.rs

# Retry crate - Retry mechanism
mkdir -p crates/retry/src/{policy,backoff,timeout,circuit,notification,recovery}
touch crates/retry/Cargo.toml
touch crates/retry/src/lib.rs
touch crates/retry/src/policy/{count,timeout,error,custom,composite,adaptive}.rs
touch crates/retry/src/backoff/{fixed,exponential,random,jitter,custom,composite}.rs
touch crates/retry/src/timeout/{fixed,adaptive,deadline,budget,custom,composite}.rs
touch crates/retry/src/circuit/{breaker,state,metric,policy,notification,recovery}.rs
touch crates/retry/src/notification/{event,listener,handler,channel,filter,router}.rs
touch crates/retry/src/recovery/{strategy,fallback,compensation,rollback,verify}.rs

# Fallback crate - Fallback handling
mkdir -p crates/fallback/src/{strategy,policy,provider,cache,notification,recovery}
touch crates/fallback/Cargo.toml
touch crates/fallback/src/lib.rs
touch crates/fallback/src/strategy/{priority,round_robin,weighted,random,custom,composite}.rs
touch crates/fallback/src/policy/{timeout,retry,circuit,quota,custom,composite}.rs
touch crates/fallback/src/provider/{static,dynamic,discovery,custom,composite,factory}.rs
touch crates/fallback/src/cache/{memory,disk,distributed,policy,invalidation,sync}.rs
touch crates/fallback/src/notification/{event,listener,handler,channel,filter,router}.rs
touch crates/fallback/src/recovery/{strategy,retry,timeout,circuit,verify,rollback}.rs

# Error crate - Error handling
mkdir -p crates/error/src/{handler,mapper,boundary,context,recovery,notification}
touch crates/error/Cargo.toml
touch crates/error/src/lib.rs
touch crates/error/src/handler/{global,local,chain,custom,composite,factory}.rs
touch crates/error/src/mapper/{code,message,cause,stack,context,translation}.rs
touch crates/error/src/boundary/{entry,exit,propagation,isolation,containment}.rs
touch crates/error/src/context/{scope,state,environment,correlation,baggage}.rs
touch crates/error/src/recovery/{strategy,retry,fallback,compensation,rollback}.rs
touch crates/error/src/notification/{event,listener,handler,channel,filter,router}.rs

# Recovery crate - Recovery handling
mkdir -p crates/recovery/src/{strategy,state,action,verification,notification,logging}
touch crates/recovery/Cargo.toml
touch crates/recovery/src/lib.rs
touch crates/recovery/src/strategy/{retry,fallback,compensation,rollback,custom}.rs
touch crates/recovery/src/state/{normal,degraded,failed,recovering,stable}.rs
touch crates/recovery/src/action/{retry,fallback,compensate,rollback,verify}.rs
touch crates/recovery/src/verification/{check,probe,test,validate,monitor}.rs
touch crates/recovery/src/notification/{event,listener,handler,channel,filter}.rs
touch crates/recovery/src/logging/{event,context,metric,trace,analyze}.rs

# Resilience crate - Resilience patterns
mkdir -p crates/resilience/src/{circuit,retry,fallback,timeout,bulkhead,cache}
touch crates/resilience/Cargo.toml
touch crates/resilience/src/lib.rs
touch crates/resilience/src/circuit/{breaker,state,metric,policy,notification}.rs
touch crates/resilience/src/retry/{policy,backoff,timeout,circuit,notification}.rs
touch crates/resilience/src/fallback/{strategy,policy,provider,cache,notification}.rs
touch crates/resilience/src/timeout/{policy,handler,recovery,notification,metric}.rs
touch crates/resilience/src/bulkhead/{semaphore,thread,queue,rejection,metric}.rs
touch crates/resilience/src/cache/{local,distributed,policy,sync,notification}.rs

# Compensation crate - Compensation handling
mkdir -p crates/compensation/src/{saga,transaction,coordinator,participant,log,recovery}
touch crates/compensation/Cargo.toml
touch crates/compensation/src/lib.rs
touch crates/compensation/src/saga/{definition,execution,coordination,recovery}.rs
touch crates/compensation/src/transaction/{manager,participant,log,recovery}.rs
touch crates/compensation/src/coordinator/{registry,scheduler,monitor,recovery}.rs
touch crates/compensation/src/participant/{action,state,notification,recovery}.rs
touch crates/compensation/src/log/{event,state,storage,query,analyze}.rs
touch crates/compensation/src/recovery/{strategy,action,verification,notification}.rs

# Thêm quyền thực thi cho script
# chmod +x tree_ngrok-p2p.sh

echo "Đã tạo xong cấu trúc dự án ngrok-p2p với chiều sâu chi tiết cho từng phân cấp!" 

echo "Cấu trúc dự án:"
tree

