FROM scratch
ADD ./jason-longshore /jason-longshore
ENTRYPOINT ["/jason-longshore"]
